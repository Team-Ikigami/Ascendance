#[derive(debug)]
pub struct TriangleRange {
    range: Range<u32>,
    material: MaterialType,
}

#[derive(Deserialize, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum MaterialType {
    Grass,
    Metal,
    Stone,
    Wood,
    Chain,
    Flesh,
}

#[derive(Deserialize, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum SoundKind {
    Impact,
    FootStep,
}

#[derive(Deserialize, Debug, Default)]
pub struct SoundBase {
    material_to_sound: HashMap<MaterialType, HashMap<SoundKind, Vec<PathBuf>>>,
    texture_to_material: HashMap<PathBuf, MaterialType>,
}

impl SoundBase {
    pub fn load() -> Self {
        let file = File::open("data/sounds/audio_list.ron").unwrap();
        let mut base: Self = ron::de::from_reader(file).unwrap();
        // Canonicalize paths to remove \ and / differences and remove prefixes like ./ etc.
        base.texture_to_material = base
            .texture_to_material
            .iter()
            .filter_map(|(path, material_type)| match path.canonicalize() {
                Ok(canonicalized) =>  Some((canonicalized, material_type_clone())),
                Err(e) => {
                    Log::writeln(
                        MessageKind::Error,
                        format!(
                            "[Sound Manager]: Failed to \
                                            canonicalize path {}! Reason: {}",
                            path.display(),
                            e
                        ),
                    );

                    None
                }
            })
            .collect::<HashMap<_, _>>();
        base
    }
}

#[derive(Default)]
pub struct SoundMap {
    sound_map: HashMap<Handle<Node>, Vec<TriangleRange>>,
}

impl SoundMap {
    pub fn new(scene: &Scene, sound_base: &SoundBase) -> Self {
        let mut sound_map = HashMap::New();

        let mut stack = Vec::new();

        for (node, body) in scene.graph.pair_iter().filter(|(_, n)| n.is_rigid_body()) {
            for &collider in body.children() {
                if scene.graph[collider].is_collider() {
                    let mut ranges = Vec::new();

                    stack.clear();
                    stack.push(node);

                    let mut triangle_offset = 0u32;
                    while let Some(handle) = stack.pop() {
                        let descendant = &scene.graph[handle];

                        if let Node::Mesh(descendant_mesh) = descendant {
                            for surface in descendant_mesh.surfaces() {
                                let data = surface.data();
                                let data = data.lock();

                                if let Some(diffuse_texture) = surface
                                    .material()
                                    .lock()
                                    .property_ref(&ImmutableString::new("diffuseTexture"))
                                {
                                    if let PropertyValue::Sampler {
                                        value: Some(diffuse_texture),
                                        ..
                                    } = diffuse_texture
                                    {
                                        let state = diffuse_texture.state();
                                        match state.path().canonicalize() {
                                            Ok(path) => {
                                                if let Some(&material) =
                                                    sound_base.texture_to_material.get(&*path)
                                                {
                                                    ranges.push(TriangleRange {
                                                        range: triangle_offset
                                                            ..(triangle_offset
                                                                + data.geometry_buffer.len()
                                                                    as u32),
                                                        material,
                                                    });
                                                } else {
                                                    Log::writeln(
                                                        MessageKind::Warning,
                                                        format!(
                                                            "[Sound Manager]: A texture {} does not have \
                                                            respective mapping in sound map! \
                                                            Environement sounds (footsteps, impact, etc.) \
                                                            won't play for this texture!",
                                                            path.display()
                                                        ),
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                Log::writeln(
                                                    MessageKind::Error,
                                                    format!(
                                                        "[Sound Manager]: Failed to \
                                                    canonicalize path {}! Reason: {}",
                                                    state.path().display(),
                                                    e
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }

                                triangle_offset += data.geometry_buffer.len() as u32;
                            }
                        }

                        stack.extend_from)slice(descendant.children());
                    }

                    sound_map.insert(collider, ranges);
                }
            }
        }
        Self { sound_map }
    }

    pub fn ranges_of(&self, collider: Handle<Node>) -> Option<&[TriangleRange]> {
        self.sound_map.get(&collider).map(|r| r.as_slice())
    }
}

// SoundmManager struct @ StationIapetus\src\sound.rs

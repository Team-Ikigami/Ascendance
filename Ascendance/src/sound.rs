use super::message::Message;

use fyrox::core::sstorage::ImmutableString;
use fyrox::material::PropertyValue;
use fyrox::scene::graph::physics::FeatureId;
use fyrox::{
    core::{algebra::Vector3, pool::Handle, visitor::prelude::*},
    engine::resource_manager::ResourceManager,
    rand::{self, seq::SliceRandom},
    scene::{
		node::Node,
		Scene,
	},
    utils::log::{Log, MessageKind},
};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, ops::Range, path::Path, path::PathBuf, time::Duration};
use fyrox::scene::sound::{
    context::SoundContext,
    effects::{BaseEffect, Effect, EffectInput},
    source::{generic::GenericSourceBuilder, spatial::SpatialSourceBuilder, Status},
};

// Used to determine how the sound is played based on the material (a deserialized set of information) and range, an unsigned 32 bit number for distance.
#[derive(Debug)]
pub struct TriangleRange {
    range: Range<u32>,
    material: MaterialType,
}

// presumably used to affect how the sound rebounds
// i copied this and have forgotten how it works
#[derive(Deserialize)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum MaterialType {
    Grass,
    Metal,
    Stone,
    Wood,
    // Chain,
    // Flesh,
}

#[derive(Deserialize)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum SoundKind {
    Impact,
    FootStep,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Default)]
pub struct SoundBase {
    material_to_sound: HashMap<MaterialType, HashMap<SoundKind, Vec<PathBuf>>>,
    texture_to_material: HashMap<PathBuf, MaterialType>,
}

impl SoundBase {
    pub fn load() -> Self {
		// Deserialize list of sounds.
        let file = File::open("data/sounds/sound_list.ron").unwrap();
        let mut base: Self = ron::de::from_reader(file).unwrap();
        // Canonicalize paths to remove \ and / differences and remove prefixes like ./ etc.
        base.texture_to_material = base
            .texture_to_material
            .iter()
            .filter_map(|(path, material_type)| match path.canonicalize() {
                Ok(canonicalized) => Some((canonicalized, material_type.clone())),
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
        let mut sound_map = HashMap::new();

        let mut stack = Vec::new();
        // performs for every node and body in the scene, filtering rigid bodies?
        for (node, body) in scene.graph.pair_iter().filter(|(_, n)| n.is_rigid_body()) {
            // perorms for every nodoe WITH a rigid body
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

                        stack.extend_from_slice(descendant.children());
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

#[derive(Default)]
#[derive(Visit)]
pub struct SoundManager {
    context: SoundContext,
    reverb: Handle<Effect>,
    #[visit(skip)]
    sound_base: SoundBase,
    #[visit(skip)]
    sound_map: SoundMap,
}

impl SoundManager {
    pub fn new(context: SoundContext, scene: &Scene) -> Self {
        let mut base_effect = BaseEffect::default();
        base_effect.set_gain(0.7);
        let mut reverb = fyrox_sound::effects::reverb::Reverb::new(base_effect);
        reverb.set_dry(0.5);
        reverb.set_wet(0.5);
        reverb.set_decay_time(Duration::from_secs_f32(3.0));
        let reverb = context
            .state()
            .add_effect(fyrox_sound::effects::Effect::Reverb(reverb));

        let sound_base = SoundBase::load();

        Self {
            context,
            reverb,
            sound_map: SoundMap::new(scene, &sound_base),
            sound_base,
        }
    }

    async fn play_sound(
        &self,
        path: &Path,
        position: Vector3<f32>,
        gain: f32,
        rolloff_factor: f32,
        radius: f32,
        resource_manager: ResourceManager,
    ) {
        // We use spatialsourcebuilder because we want our sound to be affected by the distance
        // from which the stabbing occured. I am currently using the rusty-shooter repository as an
        // example so i might consider changing it later. For now this should be fine and work.
        if let Ok(buffer) = resource_manager.request_sound_buffer(path, false).await {
            let stabbed_sound = SpatialSourceBuilder::new(
                GenericSourceBuilder::new()
                    .with_buffer(buffer.into())
                    .with_status(Status::Playing)
                    .with_play_once(true)
                    .with_gain(gain)
                    .build()
                    .unwrap(),
            )
            .with_position(position)
            .with_radius(radius)
            .with_rolloff_factor(rolloff_factor)
            .build_source();

            let mut state = self.context.state();
            let source = state.add_source(stabbed_sound);
            state
                .effect_mut(self.reverb)
                .add_input(EffectInput::direct(source));
        } else {
            Log::writeln(
                MessageKind::Error,
                format!("Unable to play sound {:?}", path),
            );
        }
    }

    pub async fn handle_message(&mut self, resource_manager: ResourceManager, message: &Message) {
        match message {
            Message::PlaySound {
                path,
                position,
                gain,
                rolloff_factor,
                radius,
            } => {
                self.play_sound(
                    path,
                    *position,
                    *gain,
                    *rolloff_factor,
                    *radius,
                    resource_manager,
                )
                .await;
            }
            &Message::PlayEnvironmentSound {
                collider,
                feature,
                position,
                sound_kind,
                gain,
                rolloff_factor,
                radius,
            } => {
                let material = self
                    .sound_map
                    .ranges_of(collider)
                    .map(|ranges| {
                        match feature {
                            FeatureId::Face(idx) => {
                                // FeatureId::Face(idx) is
                                let mut material = None;
                                for range in ranges {
                                    if range.range.contains(&idx) {
                                        material = Some(range.material);
                                        break;
                                    }
                                }
                                material
                            }
                            _ => {
                                // Some things have odd colliders, they dont provide useful info
                                // about impact locationm we have to use first available material.
                                ranges.first().map(|first_range| first_range.material)
                            }
                        }
                    })
                    .flatten();

                if let Some(material) = material {
                    if let Some(map) = self.sound_base.material_to_sound.get(&material) {
                        if let Some(sound_list) = map.get(&sound_kind) {
                            if let Some(sound) = sound_list.choose(&mut rand::thread_rng()) {
                                self.play_sound(
                                    sound.as_ref(),
                                    position,
                                    gain,
                                    rolloff_factor,
                                    radius,
                                    resource_manager,
                                )
                                .await;
                            }
                        } else {
                            Log::writeln(
                                MessageKind::Warning,
                                format!(
                                    "Unable to play environment sound there \
										is no respective mapping for {:?} sound kind!",
                                    sound_kind
                                ),
                            );
                        }
                    } else {
                        Log::writeln(
                            MessageKind::Warning,
                            format!(
                                "Unable to play environment sound: there \
									is no respective mapping for {:?} material!",
                                material
                            ),
                        );
                    }
                } else {
                    Log::writeln(
                        MessageKind::Warning,
                        "Unable to play environment sound: unable to fetch material type!"
                            .to_owned(),
                    );
                }
            }
            _ => {}
        }
    }

    pub fn resolve(&mut self, scene: &Scene) {
        self.sound_base = SoundBase::load();
        self.sound_map = SoundMap::new(scene, &self.sound_base);
    }
}

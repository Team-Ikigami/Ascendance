#![allow(clippy::too_many_arguments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod message {
	use crate::{
		sound::SoundKind,
	};
	use fyrox::core::{
		algebra::{UnitQuaternion, Vector3},
		pool::Handle,
	};
	use fyrox::scene::graph::physics::FeatureId;
	use fyrox::scene::node::Node;
	use std::path::PathBuf;
	
	#[derive(Debug)]
	pub enum Message {
	   PlaySound {
		   path: PathBuf,
		   position: Vector3<f32>,
		   gain: f32,
		   rolloff_factor: f32,
		   radius: f32,
	   },
	   PlayEnvironmentSound {
		   collider: Handle<Node>,
		   feature: FeatureId,
		   position: Vector3<f32>,
		   sound_kind: SoundKind,
		   gain: f32,
		   rolloff_factor: f32,
		   radius: f32,
	   },
	}	
}
mod player {
	mod camera {
		use fyrox::{
			core::{
				algebra::{UnitQuaternion, Vector3},
				pool::Handle,
			},
			engine::resource_manager::ResourceManager,
			event::DeviceEvent,
			resource::texture::TextureWrapMode,
			scene::{
				base::BaseBuilder,
				camera::{CameraBuilder, SkyBox, SkyBoxBuilder},
				graph::Graph,
				node::Node,
				transform::TransformBuilder,
			},
		};
		async fn create_skybox(resource_manager: ResourceManager) -> SkyBox {
			let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
				resource_manager.request_texture("data/textures/skybox/front.jpg"),
				resource_manager.request_texture("data/textures/skybox/back.jpg"),
				resource_manager.request_texture("data/textures/skybox/left.jpg"),
				resource_manager.request_texture("data/textures/skybox/right.jpg"),
				resource_manager.request_texture("data/textures/skybox/up.jpg"),
				resource_manager.request_texture("data/textures/skybox/down.jpg")
			);
			// Unwrap(?)
			let skybox = SkyBoxBuilder {
				front: Some(front.unwrap()),
				back: Some(back.unwrap()),
				left: Some(left.unwrap()),
				right: Some(right.unwrap()),
				top: Some(top.unwrap()),
				bottom: Some(bottom.unwrap()),
			}
				.build()
				.unwrap();
			// remove seams of the sky
			let cubemap = skybox.cubemap();
			let mut data = cubemap.as_ref().unwrap().data_ref();
			data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
			data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);
			skybox
		}
		pub struct CameraController {
			pivot: Handle<Node>,
			hinge: Handle<Node>,
			camera: Handle<Node>,
			yaw: f32,
			pitch: f32,
		}
		impl CameraController {
			pub async fn new (graph: &mut Graph, resource_manager: ResourceManager) -> Self {
				let camera;
				let hinge;
				let pivot = BaseBuilder::new()
					.with_children(&[{
						hinge = BaseBuilder::new()
							.with_local_transform(
								TransformBuilder::new()
									// move the hinge of the pivot & camera up to the characters head/body
									.with_local_position(Vector3::new(0.0, 0.55, 0.0))
									.build(),
								)
								.with_children(&[{
									camera = CameraBuilder::new(
										BaseBuilder::new()
											.with_local_transform(
												TransformBuilder::new()
													// move Camera to behind the characters head
													.with_local_position(Vector3::new(0.0, 0.0, -2.0))
													.build(),
											),
									)
									.with_skybox(create_skybox(resource_manager).await)
									.build(graph);
									
									camera
								}])
								.build(graph);
						hinge
					}])
					.build(graph);
				
				Self {
					pivot,
					hinge,
					camera,
					yaw: 0.0,
					pitch: 0.0,
				}
			}
			pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
				// https://howthingsfly.si.edu/flight-dynamics/roll-pitch-and-yaw very useful
				if let DeviceEvent::MouseMotion { delta } = device_event {
					const MOUSE_SENSITIVITY: f32 = 0.015;
					// Yaw is the horizontal rotation
					self.yaw -= (delta.0 as f32) * MOUSE_SENSITIVITY;
					// pitch is the vertical forward/backward rotation. Want to limit it to straight up and
					// straight down and not break the neck of the character as it looks up and backwards
					// while rotating the same direction. 
					self.pitch = (self.pitch + (delta.1 as f32) * MOUSE_SENSITIVITY)
						// Limit vertical angle to -90; 90 degrees range?
						.max(-90.0f32.to_radians())
						.min(90.0f32.to_radians());
				}
			}
			pub fn update(&mut self, graph: &mut Graph) {
				// Apply rotation to the pivot (the base node of the body-hinge-camera triangle that I dont
				// really understand. I think the pivot needs rotation to turn the character left and
				// right?
				graph[self.pivot]
					.local_transform_mut()
					.set_rotation(UnitQuaternion::from_axis_angle(
							&Vector3::y_axis(),
							self.yaw
					));
				// apply rotation to the hinge. This is the X axis which is on the same plane as the y
				// axis? I dont understand it so ill definitely inquire
				// TODO
				graph[self.hinge]
					.local_transform_mut()
					.set_rotation(UnitQuaternion::from_axis_angle(
							&Vector3::x_axis(),
							self.pitch
					));
		
				// NOTE
				// Based on what was written above, pitch seems to be the up-down vertical axis. It is
				// however described as x_axis in the directly above snippet? that feels like a big error 
			}
		}
	}
	use crate::player::camera::CameraController;
	use fyrox::{
		animation::{
			machine::{Machine, Parameter, PoseNode, State, Transition},
			Animation,
		},
		core::{
			algebra::{UnitQuaternion, Vector3},
			pool::Handle,
		},
		engine::resource_manager::ResourceManager,
		event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode},
		resource::model::Model,
		scene::{
			base::BaseBuilder,
			collider::ColliderBuilder,
			collider::ColliderShape,
			graph::physics::CoefficientCombineRule,
			node::Node,
			rigidbody::RigidBodyBuilder,
			transform::TransformBuilder,
			Scene,
			graph::Graph,
		},
	};
	mod camera;
	pub struct Player {
		model: Handle<Node>,
		camera_controller: CameraController,
	}
	impl Player {
		pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
			let model = resource_manager
				.request_model("data/models/paladin/paladin.fbx")
				.await
				.unwrap()
				.instantiate_geometry(scene);

			scene.graph[model]
				.local_transform_mut()
				// the center starts at its feet so we lower the model
				.set_position(Vector3::new(0.0, -0.75, 0.0))
				// make model smaller
				.set_scale(Vector3::new(0.02, 0.02, 0.02));
			Self {
				model,
				// Create Camera controller
				camera_controller: CameraController::new(&mut scene.graph, resource_manager).await,
			}
		}
		pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
			self.camera_controller.handle_device_event(device_event)
		}
		pub fn update(&mut self, scene: &mut Scene) {
			self.camera_controller.update(&mut scene.graph);
		}
	}
}
mod sound {
	use super::message::Message;
	use fyrox::core::sstorage::ImmutableString;
	use fyrox::material::PropertyValue;
	use fyrox::scene::graph::physics::FeatureId;
	use fyrox::{
		core::{algebra::Vector3, pool::Handle, visitor::prelude::*},
		engine::resource_manager::ResourceManager,
		rand::{self, seq::SliceRandom},
		scene::{node::Node, Scene},
		sound::{
			context::SoundContext,
			effects::{BaseEffect, Effect, EffectInput},
			source::{generic::GenericSourceBuilder, spatial::SpatialSourceBuilder, Status},
		},
		utils::log::{Log, MessageKind},
	};
	use serde::Deserialize;
	use std::{collections::HashMap, fs::File, ops::Range, path::Path, path::PathBuf, time::Duration};

	#[derive(Debug)]
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
					Ok(canonicalized) =>  Some((canonicalized, material_type.clone())),
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

	#[derive(Default, Visit)]
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
			let mut reverb = fyrox::sound::effects::reverb::Reverb::new(base_effect);
			reverb.set_dry(0.5);
			reverb.set_wet(0.5);
			reverb.set_decay_time(Duration::from_secs_f32(3.0));
			let reverb = context
				.state()
				.add_effect(fyrox::sound::effects::Effect::Reverb(reverb));

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
}
// mod ui;
mod world;
mod entitygen;
mod loading_screen;
mod inventory;
mod weapon {
	// attack types. Used by enemies and player.
	#[derive(Debug, Copy, Clone)]
	pub struct AttackTypes {
		pub fire: bool,
		pub water: bool,
		pub cold: bool,
		pub lightning: bool,
		pub poison: bool,
		pub psychic: bool,
		pub acid: bool,
		pub necrotic: bool,
		pub bash: bool,
		pub piercing: bool,
		pub slashing: bool,
	}
}
mod door;
mod light;
mod config;
mod save_load;
mod level {
	use fyrox::{
		core::pool::Handle,
		engine::resource_manager::{ResourceManager},
		scene::{node::Node, Scene},
	};
	pub struct Level {
		root: Handle<Node>,
	}
	// creates a ne resources by requesting the level.rgs file and loading it to the scene.
	impl Level {
		pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
			let root = resource_manager
				.request_model("data/levels/level.rgs")
				.await
				.unwrap()
				.instantiate_geometry(scene);
			Self {
				root
			}
		}
	}	
}
use crate::{level::Level, player::Player};
use fyrox::{
    window::{Icon, Fullscreen},
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        widget::{WidgetBuilder, WidgetMessage},
        grid::{GridBuilder, GridDimension},
        check_box::CheckBoxBuilder,
        image::ImageBuilder,
        scroll_bar::ScrollBarBuilder,
        text::TextBuilder,
        text_box::{TextBoxBuilder, TextBoxMessage},
	menu::{MenuMessage, MenuItemMessage},
        message::{
            UiMessage,
            MessageDirection,
            MouseButton,
            MessageData,
        },
        menu::{MenuBuilder, MenuItemBuilder, MenuItemContent},
        DEFAULT_FONT,
        DragContext,
        MouseState,
        Thickness,
        UserInterface,
	UiNode
    },
    core::{
        pool::{Handle, PoolIterator, PoolIteratorMut},
        color::Color,
        futures::executor::block_on,
    },
    asset::{
        define_new_resource, 
        Resource, 
        ResourceLoadError, 
        ResourceData, 
        ResourceState
    },
    event::{WindowEvent, DeviceId, DeviceEvent},
    utils::into_gui_texture,
    event_loop::ControlFlow,
    scene::Scene,
};
use rg3d_sound::{
    source::{generic::GenericSourceBuilder, SoundSource, Status},
    engine::SoundEngine,
    context::SoundContext,
    buffer::{DataSource, SoundBufferResource},
    pool::Handle as CoreSoundHandle,
};
use std::{
    thread,
    time::Duration
};
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use rand::Rng;
use image::io::Reader as ImageReader;

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

// Structs

struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
}

// fn Newgame() {}
// fn Settings() {}
// fn Exit() {}
// fn Savegame() {}
// fn Loadgame() {}
// fn Inventory() {}

// fn BasicBandit(builder: &mut ResourceManager) {
//     let build = builder.request_model();
//     build("assets/model/BasicBandit.fbx")
// }
// fn BasicBanditWarlock(builder: &mut ResourceManager) {}
// fn BasicChief(builder: &mut ResourceManager) {}
// fn AverageBandit(builder: &mut ResourceManager) {}
// fn AverageBanditWarlock(builder: &mut ResourceManager) {}
// fn AverageBanditBarbarian(builder: &mut ResourceManager) {}
// fn AverageBanditChief(builder: &mut ResourceManager) {}
fn bgmloop() {
    let engine = SoundEngine::new();
    let context = SoundContext::new();
    engine.lock().unwrap().add_context(context.clone());
    let mut randbgmint = rand::thread_rng().gen_range(1..6);
    match randbgmint {
        1 => {
            let sound_buffer = SoundBufferResource::new_generic(rg3d_sound::futures::executor::block_on(DataSource::from_file("data/music/themetest.wav"))
                .unwrap())
                .unwrap();
            
            let source = GenericSourceBuilder::new()
                .with_buffer(sound_buffer)
                .with_status(Status::Playing)
                .build_source()
                .unwrap();
            let _source_handle: CoreSoundHandle<SoundSource> = context.state().add_source(source);
            
            thread::sleep(Duration::from_secs(17));
        }
        2 => bgmloop(),
        3 => bgmloop(),
        4 => bgmloop(),
        5 => bgmloop(),
        _ => bgmloop(),
    }
    bgmloop();
}

//        let ctx = &mut engine.user_interface.build_ctx();
//        let audioengine = SoundEngine::new();
//        let soundcontenttest = SoundContext::new();
//        audioengine.lock().unwrap().add_context(soundcontenttest.clone());
//        let sound_buffer_test = SoundBufferResource::new_generic(rg3d_sound::futures::executor::block_on(DataSource::from_file("data/music/themetest.wav")).unwrap()).unwrap();
//        let sourcetest = GenericSourceBuilder::new()
//            .with_buffer(sound_buffer_test)
//            .with_looping(true)
//            .with_status(Status::Playing)
//            .build_source()
//            .unwrap();
//        let _source_handle: CoreSoundHandle<SoundSource> = soundcontenttest.state().add_source(sourcetest);
//        thread::sleep(Duration::from_secs(17));


/// Uses the rg3d crates framework requirements. This section runs all the necesary functions and such.
/// init is used for initializing of the game. In here we have resoure checks, preloading of the game and other stuff.
/// on_tick is used for logic that happens every second. It has a fixed fps of 60 stored in the dt variable.
/// on_ui_message handles the messges created from interacting with the ui. if let some() statements are exetremely common here.
/// on_device_event handles the device events such as mouse and keyboard.
/// on_window_event handles the window events such as resizing.
/// on_resource_load handles the resource loading.
impl GameState for Game {
    fn init(engine: &mut Engine) -> Self 
        where 
            Self: Sized 
    {
        let mut scene = Scene::new();
        scene.ambient_lighting_color = Color::opaque(150, 150, 150);
        let player = block_on(Player::new(engine.resource_manager.clone(), &mut scene));
        let img = ImageReader::open("data/textures/icons/window_icon.png")
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let mut pixels = Vec::new();
        img.pixels().for_each(|pixel| {
            pixel.0.iter().for_each(|byte| {
                pixels.push(*byte);
            })
        });
        engine.get_window()
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
        engine.get_window()
            .set_window_icon(Some(Icon::from_rgba(pixels, img.width(), img.height()).unwrap()));
        engine.get_window()
            .set_cursor_visible(false);
        engine.get_window()
            .set_cursor_grab(true);
        engine.get_window()
            .set_resizable(false);
        Self {
            player,
            level: block_on(Level::new(engine.resource_manager.clone(), &mut scene)),
            scene: engine.scenes.add(scene),
        }
    }
	fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {
            let scene = &mut engine.scenes[self.scene];
            self.player.update(scene);
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
	fn on_device_event(
            &mut self,
            _engine: &mut Engine,
            _device_id: DeviceId,
            event: DeviceEvent
        ) {
            self.player.handle_device_event(&event);
        }
	fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent) {}
	fn on_exit(&mut self, engine: &mut Engine) {}
}

/// Uses the rg3d crate to create a window and run the game loop.
fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}

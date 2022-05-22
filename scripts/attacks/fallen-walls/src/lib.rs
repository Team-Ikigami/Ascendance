use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        inspect::{Inspect, PropertyInfo},
        pool::Handle,
        uuid::{Uuid, uuid},
        visitor::prelude::*,
    },
    event::{ElementState, Event},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{
        node::{Node, TypeUuidProvider},
        rigidbody::RigidBody,
    },
    script::{ScriptContext, ScriptTrait}
};
use fyrox::engine::{resource_manager::ResourceManager, Engine};
use fyrox::scene::rigidbody::RigidBodyBuilder;
use fyrox::scene::{
    collider::ColliderBuilder,
	particle_system::ParticleSystemBuilder,
    Scene,
};
use std::str::FromStr;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::transform::TransformBuilder;
use fyrox::scene::collider::ColliderShape;
use fyrox::event::DeviceEvent;
use fyrox::event::VirtualKeyCode;
use fyrox::event::KeyboardInput;
use fyrox::scene::graph::Graph;
use fyrox::event::WindowEvent;

#[derive(Visit, Default, Inspect)]
struct GamePlugin {
    scene: Handle<Scene>,
}

impl Plugin for GamePlugin {
	fn on_register(&mut self, context: PluginRegistrationContext) {
		// Necessary to use it
	}
	fn on_standalone_init(&mut self, context:PluginContext) {
		// Executor
	}
	fn on_enter_play_mode(&mut self, scene: Handle<Scene>, context: PluginContext) {
		async fn goddamnit_compiler(event: DeviceEvent){
			// await if let event = Some(Keyboard);
                        println!("Stop");
		}
	}
	fn on_leave_play_mode(&mut self, context: PluginContext) {
		// Used in editor the moment play_mode is stopped
	}
	fn on_unload(&mut self, context: &mut PluginContext) {
		// Used when editor is about to shut down. engine::framework::Framework::on_exit()
	}
	fn update(&mut self, context: &mut PluginContext) {
		// Happens every frame. engine::framework::Framework::update()
	}
	fn id(&self) -> Uuid {
		uuid!("a9507fb2-0945-4fc1-91c3-115ae7c8a615")
	}
	fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) {
		// The method is called when the main window receives an event from the OS.
		async fn i_fucking_hate_compiler(event: &Event<()>, resource_manager: ResourceManager, scene: &mut Scene) {
			match event {
				Event::WindowEvent { event, .. } => {
					if let WindowEvent::KeyboardInput { input, .. } = event {
						if let Some(key_code) = input.virtual_keycode {
							match key_code {
								VirtualKeyCode::F => {
									let rock_one = resource_manager.request_model("data/models/rock/rock1.fbx").await.unwrap().instantiate_geometry(scene);
									scene.graph[rock_one]
										.local_transform_mut()
										.set_position(Vector3::new(10.0, 50.0, 10.0))
										.set_scale(Vector3::new(0.05, 0.05, 0.05));
									let rock_collider;
									let rigid_body = RigidBodyBuilder::new(
										BaseBuilder::new()
											.with_local_transform(
												TransformBuilder::new()
													.with_local_position([10.0, 50.0, 10.0])
													.build(),
											)
											.with_children(&[
												rock_one,
												{
													rock_collider = ColliderBuilder::new(BaseBuilder::new())
															.with_shape(ColliderShape::capsule_y(0.5, 0.5))
															.build(&mut scene.graph);
													rock_collider
												}
											])
												
									)
									.with_lin_vel(10.0)
									.with_ccd_enabled(true)
									.with_ang_vel(30.0)
									.with_can_sleep(false)
									.build(&mut Graph);
								}
							}
						}
					}
				}
			}
		}
	}
}

use fyrox::*;//{
//	core::{
//		algebra::{UnitQuaternion, Vector3},
//		inspect::{Inspect, PropertyInfo},
//		pool::Handle,
//		uuid::Uuid,
//		visitor::prelude::*,	
//	},
//	event::{ElementState, Event},
//	plugin::{Plugin, PluginContext},
//	scene::{
//		node::{Node, TypeUuidProvider},
//		rigidbody::RigidBody,
//	},
//	script::{ScriptContext, ScriptTrait}
//};
use fyrox::engine::{Engine, resource_manager::ResourceManager};
use fyrox::scene::{
	particle_system::ParticleSystemBuilder,
	Scene,
	rigidbody::RigidBodyBuilder,
	collider::ColliderBuilder,
};
use fyrox::core::pool::Handle;
use fyrox::asset::core::pool::Handle;
use fyrox::scene::rigidbody::RigidBodyBuilder;
use std::str::FromStr;

#[derive(Visit, Default, Inspect)]
struct GamePlugin {
	scene: Handle<Scene>,
}

impl GamePlugin {
	fn new(
		scene: &mut Scene,
		position: Vector3<f32>,
		resource_manager: ResourceManager,
	) -> Self {
		let rock_1 = resource_manager
			.request_model("data/models/rock_1.fbx")
			.await
			.unwrap()
			.request_texture("data/models/rocks/one.jpg")
			.await
			.unwrap()
			.instantiate_geometry(scene);
		scene.graph([rock_1])
			.local_transform_mut()
			// set position to a fairly high area
			.set_position(Vector3::new(0.0, 5.0, 0.0))
			.set_scale(Vector3::new(0.5, 0.5, 0.5))
		let collider;
		let rigid_body = RigidBodyBuilder(
			BaseBuilder::new()
				.with_local_transform(
					TransformBuilder::new()
						.with_local_position(Vector3::new(position.x, position.y, position.z))
						.build(),
				)
				.with_children(&[
					// attach the model to the rigidbody
					rock_1,
					// add capsule collider to the rigidbody
					{
						collider = ColliderBuilder::new(BaseBuilder::new())
							.with_shape(ColliderShape::capsule_y(0.25,0.2))
							.build(&mut scene.graph);
						collider
					},
				]),
		)

		// we want the rock to tilt and move everywhere
		.with_locked_rotations(false)
		.with_can_sleep(false)
		.build(&mut scene.graph);

		Self {
			collider,
			rigid_body
		}
	}
}


impl TypeUuidProvider for GamePlugin {
	fn type_uuid() -> Uuid {
		Uuid::from_str("a9507fb2-0945-4fc1-91c3-115ae7c8a615").unwrap()
	}
}
#[no_mangle]
pub extern "C" fn fyrox_main() -> Box<Box<dyn Plugin>> {
	Box::new(Box::new(GamePlugin::new()))
}

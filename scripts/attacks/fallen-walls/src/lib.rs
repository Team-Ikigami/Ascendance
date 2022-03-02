use fyrox::{
	core::{
		algebra::{UnitQuaternion, Vector3},
		inspect::{Inspect, PropertyInfo},
		pool::Handle,
		uuid::Uuid,
		visitor::prelude::*,	
	},
	event::{ElementState, Event},
	plugin::{Plugin, PluginContext},
	scene::{
		node::{Node, TypeUuidProvider},
		rigidbody::RigidBody,
	},
	script::{ScriptContext, ScriptTrait}
};
use std::str::FromStr;

#[derive(Visit, Default, Inspect)]
struct GamePlugin {}

impl GamePlugin {
	pub fn new() -> Self {
		Self {}
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
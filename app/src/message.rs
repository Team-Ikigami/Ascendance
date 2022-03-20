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
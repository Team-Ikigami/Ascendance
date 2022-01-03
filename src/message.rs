use crate::{
    sound::SoundKind,
};
use rg3d::core::{
    algebra::{UnitQuaternion, Vector3},
    pool::Handle,
};
use rg3d::scene::graph::physics::FeatureId;
use rg3d::scene::node::Node;
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

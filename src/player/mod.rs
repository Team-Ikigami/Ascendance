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
}

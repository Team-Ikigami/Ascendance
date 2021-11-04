use crate::player::camera::CameraController;

// Import everything we need for the tutorial.
use rg3d::{
    animation::{
        machine::{Machine, Parameter, PoseNode, State, Transition},
        Animation,
    },
    core::{
        algebra::{Isometry3, UnitQuaternion, Vector3},
        pool::Handle,
    },
    engine::{
        resource_manager::{MaterialSearchOptions, ResourceManager},
        ColliderHandle, RigidBodyHandle,
    },
    event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode},
    physics::{
        dynamics::{CoefficientCombineRule, RigidBodyBuilder},
        geometry::ColliderBuilder,
    },
    resource::model::Model,
    scene::{base::BaseBuilder, node::Node, Scene},
};

mod camera;

pub struct Player {
    pivot: Handle<Node>,
    model: Handle<Node>,
    camera_controller: CameraController,
}

impl Player {
    pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
        // Create pivot for our character.
        let pivot = BaseBuilder::new().build(&mut scene.graph);

        // Load paladin 3D model and create its instance in the scene.
        let model = resource_manager
            .request_model(
                "data/models/paladin/paladin.fbx",
                MaterialSearchOptions::RecursiveUp,
            )
            .await
            .unwrap()
            .instantiate_geometry(scene);

        scene.graph[model]
            .local_transform_mut()
            // Move the model a bit down because its center is at model's feet
            // and we'd get floating model without this offset.
            .set_position(Vector3::new(0.0, -0.75, 0.0))
            // Scale down paladin's model because it is too big. 
            .set_scale(Vector3::new(0.02, 0.02, 0.02));

        // Finally attach the model to the pivot. This will force model to move together with the pivot.
        scene.graph.link_nodes(model, pivot);

        Self {
            pivot,
            model,

            // As a final stage create camera controller.
            camera_controller: CameraController::new(&mut scene.graph, resource_manager).await,
        }
    }
}

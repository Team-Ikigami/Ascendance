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
    input_controller: InputController,
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
        // Create new rigid body and offset it a bit to prevent falling through the ground.
        let body = scene.physics.add_body(
            RigidBodyBuilder::new_dynamic()
                .position(Isometry3::translation(0.0, 2.0, 0.0))
                .build(),
        );

        // Create capsule collider with friction disabled. We need to disable friction because linear
        // velocity will be set manually, but the physics engine will reduce it using friction so it
        // won't let us to set linear velocity precisely.
        let capsule = ColliderBuilder::capsule_y(0.55, 0.25)
            .friction_combine_rule(CoefficientCombineRule::Min)
            .friction(0.0)
            .build();
        let collider = scene.physics.add_collider(capsule, &body);

        // Finally bind the pivot with the body.
        scene.physics_binder.bind(pivot, body);

        Self {
            pivot,
            model,
            // As a final stage create camera controller.
            camera_controller: CameraController::new(&mut scene.graph, resource_manager).await,
            input_controller: Default::default(),
        }
    }
    pub fn handle_device_event(&mut self, device_event: &DeviceEvent) {
        self.camera_controller.handle_device_event(device_event)
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.camera_controller.update(&mut scene.graph);
        let pivot = &scene.graph[self.pivot];
        let look_vector = pivot
            .look_vector()
            .try_normalize(f32::EPSILON)
            .unwrap_or(Vector3::z());
        let side_vector = pivot
            .side_vector()
            .try_normalize(f32::EPSILON)
            .unwrap_or(Vector3::x());
        let position = **pivot.local_transform().position();
        let mut velocity = Vector3::default();
        if self.input_controller.walk_right {
            velocity -= side_vector;
        }
        if self.input_controller.walk_left {
            velocity += side_vector;
        }
        if self.input_controller.walk_forward {
            velocity += look_vector;
        }
        if self.input_controller.walk_backward {
            velocity -= look_vector;
        }
        let speed = 1.35 * dt;
        let velocity = velocity
            .try_normalize(f32::EPSILON)
            .and_then(|v| Some(v.scale(speed)))
            .unwrap_or(Vector3::default());
        let body = scene.physics.bodies.get_mut(&self.body).unwrap();
        // Apply linear velocity.
        body.set_linvel(
            Vector3::new(velocity.x / dt, body.linvel().y, velocity.z / dt),
            true,
        );
        // Lock any angular movement of the player's body.
        body.set_angvel(Default::default(), true);
        let is_moving = velocity.norm_squared() > 0.0;
        if is_moving {
            // Since we have free camera while not moving, we have to sync rotation of pivot
            // with rotation of camera so character will start moving in look direction.
            let mut current_position = *body.position();
            current_position.rotation =
                UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.camera_controller.yaw);
            body.set_position(current_position, true);
            // Apply additional rotation to model - it will turn in front of walking direction.
            let angle: f32 = if self.input_controller.walk_left {
                if self.input_controller.walk_forward {
                    45.0
                } else if self.input_controller.walk_backward {
                    135.0
                } else {
                    90.0
                }
            } else if self.input_controller.walk_right {
                if self.input_controller.walk_forward {
                    -45.0
                } else if self.input_controller.walk_backward {
                    -135.0
                } else {
                    -90.0
                }
            } else if self.input_controller.walk_backward {
                180.0
            } else {
                0.0
            };

            scene.graph[self.model]
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle.to_radians()));
        }

        // Sync camera controller position with player's position.
        scene.graph[self.camera_controller.pivot]
            .local_transform_mut()
            .set_position(position + velocity);
    }
    pub fn handle_key_event(&mut self, key: &KeyboardInput) {
        if let Some(key_code) = key.virtual_keycode {
            match key_code {
                VirtualKeyCode::W => {
                    self.input_controller.walk_forward = key.state == ElementState::Pressed
                }
                VirtualKeyCode::S => {
                    self.input_controller.walk_backward = key.state == ElementState::Pressed
                }
                VirtualKeyCode::A => {
                    self.input_controller.walk_left = key.state == ElementState::Pressed
                }
                VirtualKeyCode::D => {
                    self.input_controller.walk_right = key.state == ElementState::Pressed
                }
                _ => (),
            }
        }
    }
}

use fyrox::scene::pivot::PivotBuilder;
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
    pub async fn new(graph: &mut Graph, resource_manager: ResourceManager) -> Self {
        let camera;
        let hinge;
        let pivot = PivotBuilder::new(BaseBuilder::new()
            .with_children(&[{
                hinge = PivotBuilder::new(BaseBuilder::new()
                    .with_local_transform(
                        TransformBuilder::new()
                            // move the hinge of the pivot & camera up to the characters head/body
                            .with_local_position(Vector3::new(0.0, 0.55, 0.0))
                            .build(),
                    )
                    .with_children(&[{
                        camera = CameraBuilder::new(
                            BaseBuilder::new().with_local_transform(
                                TransformBuilder::new()
                                    // move Camera to behind the characters head
                                    .with_local_position(Vector3::new(0.0, 0.0, -2.0))
                                    .build(),
                            ),
                        )
                        .with_z_far(48.0)
                        .with_skybox(create_skybox(resource_manager).await)
                        .build(graph);
                        camera
                    }]))
                    .build(graph);
                hinge
            }]))
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
                self.yaw,
            ));
        // apply rotation to the hinge. This is the X axis which is on the same plane as the y
        // axis? I dont understand it so ill definitely inquire
        // TODO
        graph[self.hinge]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                self.pitch,
            ));

        // NOTE
        // Based on what was written above, pitch seems to be the up-down vertical axis. It is
        // however described as x_axis in the directly above snippet? that feels like a big error
    }
}

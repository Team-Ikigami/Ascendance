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
        resource_manager.request_texture("data/textures/skybox/down.jpg"),
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
                        ),
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
        }
    }
}

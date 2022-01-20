e fyrox::{
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

// https://fyrox-book.github.io/fyrox/tutorials/rpg/tutorial-1/tutorial-part-1.html#player-and-camera-controller
// part 2

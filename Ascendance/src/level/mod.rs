use fyrox::{
    core::pool::Handle,
    engine::resource_manager::ResourceManager,
    scene::{node::Node, Scene},
};
pub struct Level {
    root: Handle<Node>,
}
// creates a ne resources by requesting the level.rgs file and loading it to the scene.
impl Level {
    pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
        let root = resource_manager
            .request_model("data/levels/level.rgs")
            .await
            .unwrap()
            .instantiate_geometry(scene);
        Self { root }
    }
}
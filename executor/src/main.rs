use ascendance_lib::GamePlugin;
use doors::GamePlugin as DoorsPlugin;
use fallen_walls::GamePlugin as FallenWallsPlugin;
use fyrox::engine::executor::Executor;
use humanoids::GamePlugin as HumanoidsPlugin;
use meteor_shower::GamePlugin as MeteorShowerPlugin;
use organ_pipes::GamePlugin as OrganPipesPlugin;
use summoning_giant::GamePlugin as GiantSummoningPlugin;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin(GamePlugin::new());
    executor.add_plugin(FallenWallsPlugin::new());
    executor.add_plugin(MeteorShowerPlugin::new());
    executor.add_plugin(HumanoidsPlugin::new());
    executor.add_plugin(DoorsPlugin::new());
    executor.add_plugin(OrganPipesPlugin::new());
    executor.add_plugin(GiantSummoningPlugin::new());
    executor.run()
}

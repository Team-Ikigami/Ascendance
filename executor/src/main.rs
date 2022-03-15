use fyrox::engine::executor::Executor;
use ascendance_lib::GamePlugin;
use fallen_walls::GamePlugin as FallenWallsPlugin;
use meteor_shower::GamePlugin as MeteorShowerPlugin
use humanoids::GamePlugin as HumanoidsPlugin; 
use doors::GamePlugin as DoorsPlugin;
use organ_pipes::GamePlugin as OrganPipesPlugin;
use summoning_giant::GamePlugin as GiantSummoningPlugin;

fn main() {
	let mut executor = Executor: :new();
	executor.add_plugin(GamePlugin::new());
	executor.add_plugin(FallenWallsPlugin::new());
	executor.add_plugin(MeteorShowerPlugin::new());
	executor.add_plugin(HumanoidsPlugin::new());
	executor.add_plugin(DoorsPlugin::new());
	executor.add_plugin(OrganPipesPlugin::new());
	executor.add_plugin(GiantSummoningPlugin::new());
	executor.run()
}
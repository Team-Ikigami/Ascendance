use doors::GamePlugin as DoorsPlugin;
use fallen_walls::GamePlugin as FallingWallsPlugin;
use fyrox::event_loop::EventLoop;
use fyroxed::{Editor, StartupData};
use humanoids::GamePlugin as HumanoidsPlugin;
use meteor_shower::GamePlugin as MeteorShowerPlugin;
use organ_pipes::GamePlugin as OrganPlugin;
use summoning_giant::GamePlugin as GiantSummonPlugin;

fn main() {
    let event_loop = EventLoop::new();
    let mut editor = Editor::new(
        &event_loop,
        Some(StartupData {
            working_directory: Default::default(),
            scene: "data/scene.rgs".into(),
        }),
    );
    editor.add_game_plugin(FallingWallsPlugin::new());
    editor.add_game_plugin(MeteorShowerPlugin::new());
    editor.add_game_plugin(HumanoidsPlugin::new());
    editor.add_game_plugin(DoorsPlugin::new());
    editor.add_game_plugin(OrganPlugin::new());
    editor.add_game_plugin(GiantSummonPlugin::new());
    editor.run(event_loop);
}

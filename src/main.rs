use crate::{level::Level, player::Player};
use rg3d::{
    core::{color::Color, futures::executor::block_on, pool::Handle},
    engine::framework::{Framework, GameEngine, GameState},
    event::{DeviceEvent, DeviceId, WindowEvent},
    scene::Scene,
};
use CreatureLib::{mountain, savannha, everywhere, ocean, underground, banditfort};
mod level;
mod player;
mod gui;
struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
}
#[derive(Default)]
struct InputController {
    walk_forward: bool,
    walk_backward: bool,
    walk_left: bool,
    walk_right: bool,
}
impl GameState for Game {
    fn init(engine: &mut GameEngine) -> Self
    where
        Self: Sized,
    {}
    fn on_tick(&mut self, engine: &mut GameEngine, dt: f32) {}

    fn on_device_event(
        &mut self,
        _engine: &mut GameEngine,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {}
    fn on_window_event(&mut self, _engine: &mut GameEngine, event: WindowEvent) {}
}
fn main() {
    Framework::<Game>::new().unwrap().title("Ascending to Godhood").run()
}

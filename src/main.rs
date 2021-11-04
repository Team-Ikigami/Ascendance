use rg3d::{
    core::{color::Color, futures::executor::block_on, pool::Handle},
    engine::framework::{Framework, GameEngine, GameState},
    event::{DeviceEvent, DeviceId, WindowEvent},
    scene::Scene,
};

struct Game {
    // Empty for now.
}

impl GameState for Game {
    fn init(engine: &mut GameEngine) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn on_tick(&mut self, engine: &mut GameEngine, dt: f32) {
        // This method is called at fixed rate of 60 FPS.
        // It will contain all the logic of the game.
    }
}

fn main() {
    Framework::<Game>::new().unwrap().title("RPG").run()
}

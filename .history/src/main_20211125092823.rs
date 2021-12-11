use rg3d::{
    engine::Engine,
    engine::framework::prelude::*,
};

struct Game { }

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self where Self: Sized {
        Self { }
    }
}

fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}


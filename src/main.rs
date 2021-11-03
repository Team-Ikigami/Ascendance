// #[allow(unused_imports)]
mod items;
mod user;
mod GUI;
use savefile::*;
use ron::*;
use rg3d::{
    engine::Engine,
    engine::framework::prelude::*,
    window::Fullscreen,
};
use std::time::Instant;

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

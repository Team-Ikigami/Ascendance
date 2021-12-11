use rg3d::{
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
    gui::{
        widget::WidgetBuilder,
        text::TextBuilder,
        image::ImageBuilder,
        grid::GridBuilder,
    },
};
use std::{
    fs::File,
};
struct Game { }
fn gi(engine: &mut Engine) -> Game {
    let ctx = &mut engine.user_interface.build_ctx();
    let ui = GridBuilder::new(
        WidgetBuilder::new()
            .background(Brush::Solid(Color::WHITE))
            .with_children(
                TextBuilder::new(WidgetBuilder::new().on_row(row: usize))
                    .text("Sending recent actions log & crash error to server")
                    .build(ctx),
                GridBuilder::new(
                    WidgetBuilder::new()
                        .on_row(3)
                        .on_column(2)
                        .with_children(
                            ImageBuilder::new(WidgetBuilder::new().on_column(2))
                                .image_path("assets/crash/gif.gif")
                                .build(ctx),
                        )
                )
                .build(ctx),

            )
    )
    .add_rows(5)
    .add_columns(3)
    .build(ctx);
    ui
}
impl GameState for Game {
    fn init(engine: &mut Engine) -> Self 
        where 
            Self: Sized 
    {
        gi(engine);
        Self {}
    }
}
fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}
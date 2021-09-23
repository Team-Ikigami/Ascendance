// adds child directories
mod testsearch;
mod render;

//tells intellisense to ignore dead code
#[allow(dead_code)]

// obvious
fn main() {
    render::npc::render::when_to_render();
    render::npc::render::character::jimmy_the_candyman();
    render::items::rendered_the_items();
    testsearch::bruh::yothatswhatimlookingfor();
}

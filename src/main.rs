mod render;
mod testsearch::bruh;
#[allow(dead_code)]
fn main() {
    render::npc::render::when_to_render();
    render::npc::render::character::jimmy_the_candyman();
    render::items::rendered_the_items();
    yothatswhatimlookingfor();
}

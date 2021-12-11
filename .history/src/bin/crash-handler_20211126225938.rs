use rg3d::{
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
};
struct Game { }
impl GameState for Game {
    fn init(engine: &mut Engine) -> Self 
        where 
            Self: Sized 
    {
        let ctx = &mut engine.user_interface.build_ctx();
        OpeningUI(ctx);
        Self { }
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == SendReportUi.cancel {
                // close application
                engine.quit();
            }
    }
}
fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}
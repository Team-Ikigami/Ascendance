use rg3d::engine::Engine;
use rg3d::gui::{UserInterface, widget::WidgetBuilder, grid::GridBuilder};
fn LoadGameUI(ui: &mut UserInterface) {
    ctx = ui.build_ctx();
    let lgui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
}
fn NewGameUI(ui: &mut UserInterface) {
    ctx = ui.build_ctx();
    let ngui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
}
fn settings_ui(ui: &mut UserInterface) {
    ctx = ui.build_ctx();
    let sui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
    
}
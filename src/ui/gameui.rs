use rg3d::engine::Engine;
use rg3d::gui::{
	UserInterface,
	widget::WidgetBuilder,
	grid::GridBuilder
};
fn LoadGameUI(ui: &mut UserInterface) {
    let mut ctx = ui.build_ctx();
    let lgui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
}
fn NewGameUI(ui: &mut UserInterface) {
    let mut ctx = ui.build_ctx();
    let ngui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
}
fn settings_ui(ui: &mut UserInterface) {
    let mut ctx = ui.build_ctx();
    let sui = GridBuilder::new(WidgetBuilder::new().on_row(2).on_column(2));
    
}
struct OpeningUI
impl OpeningUI {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
		let ctx = &mut ui.build_ctx();
		GridBuilder::new(
			WidgetBuilder::new()
				.with_back(
					ImageBuilder::new(
						WidgetBuilder::new()
							.on_row(1)
							.on_column(1),
					)
					.with_texture(into_gui_texture(resource_manager.request_texture("assets/misc/opening.gif")))
					.build(ctx),
				)
				.with_child(
					newgame = MenuBuilder::new(WidgetBuilder::new().with_vertical_alignment(VerticalAlignment::Center).with_horizontal_alignment(HorizontalAlignment::Center))
						.with_items({
							let newgame = MenuItemBuilder::new(WidgetBuilder::new()
									.on_row(1)
									.on_column(1)
									.with_vertical_alignment(VerticalAlignment::Center),
							)
							.with_content(text("New Game").shortcut("NewgameUI").icon("assets/textures/widgetbackgrounds/newgame.png"))
							.with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
							.build(ctx);
							newgame;
							let setting = MenuItemBuilder::new(
								WidgetBuilder::new()
									.on_row(0)
									.on_column(0)
									.with_vertical_alignment(VerticalAlignment::Center),
							)
							.with_content(text("Settings").shortcut("SettingsUI").icon("/assets/textures/icons/settings.png"))
							.with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
							.build(ctx);
							setting;
							let exit = MenuItemBuilder::new(
								WidgetBuilder::new()
									.on_row(0)
									.on_column(1)
									.with_vertical_alignment(VerticalAlignment::Center),
							)
							.with_content(text("Exit").shortcut("").icon("assets/textures/icons/exitdoor.png"))
							.with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
							.build(ctx);
							exit;
						})
						.build(ctx),
				)
		)
		.add_row(Row::strict(200.0))
		.add_column(Column::strict(600.0))
		.build(ctx);
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}

use rg3d::engine::Engine;
use rg3d::core::Handle;
use rg3d::utils::into_gui_texture;
use rg3d::gui::{
    widget::WidgetBuilder,
    grid::{
        GridBuilder,
        Row,
        Column,
    },
    image::ImageBuilder,
    menu::{MenuItemBuilder, MenuBuilder},
    node::UiNode,
    VerticalAlignment,
    HorizontalAlignment,
    UserInterface,
};
struct ThreeCoreBasicBrewingTable;
impl ThreeCoreBasicBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl ThreeCoreMediumBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl ThreeCoreAdvancedBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl ThreeCoreGodlyBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl FiveCoreBasicBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl FiveCoreMediumBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl FiveCoreAdvancedBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl FiveCoreGodlyBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl TenCoreBasicBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl TenCoreMediumBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl TenCoreAdvancedBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl TenCoreGodlyBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl MetalBasicAnvil {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();
    }
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl MetalMediumAnvil {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl MetalAdvancedAnvil {
	fn new(ui: &mut UserInterface, self: &mut Self) -> Handle<UiNode> {
		let ctx = &mut ui.build_ctx();
		GridBuilder::new(WidgetBuilder::new().with_vertical_alignment(VerticalAlignment::Center).with_horizontal_alignment(HorizontalAlignment::Center))
		.with_child(
			HANDLE::None,
		)
		.with_background(
			ImageBuilder::new(
				WidgetBuilder::new()
					.on_row(0)
					.on_column(0)
					.with_vertical_alignment(VerticalAlignment::Center)
					.with_horizontal_alignment(HorizontalAlignment::Center),
			)
			.with_texture("assets/textures/icons/anvil.png")
			.build(ctx),
		)
		.add_columns(2)
		.add_rows(2)
		.build(ctx);
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl MetalGodlyAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl RefinedBasicAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl RefinedMediumAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl RefinedAdvancedAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl RefinedGodlyAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl NobleBasicAnvil {
	fn new(ui: &mut UserInterface) -> Handle<UiNode> {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl NobleMediumAnvil {
	fn new(ui: &mut UserInterface) {
	    let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl NobleAdvancedAnvil {
	fn new(ui: &mut UserInterface) {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
impl NobleGodlyAnvil {
	fn new(ui: &mut UserInterface) {
    	let ctx = &mut ui.build_ctx();
	}
	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
}
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

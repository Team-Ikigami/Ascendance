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
struct ThreeCoreBasicBrewingTable {
	icon: Handle<UiNode>,
	slot_1: Handle<UiNode>,
	slot_2: Handle<UiNode>,
	slot_3: Handle<UiNode>,
	fuel_slot: Handle<UiNode>,
	base: Handle<UiNode>,
}
struct ThreeCoreMediumBrewingTable;
struct ThreeCoreAdvancedBrewingTable;
struct ThreeCoreGodlyBrewingTable;

struct FiveCoreBasicBrewingTable;
struct FiveCoreMediumBrewingTable;
struct FiveCoreAdvancedBrewingTable;
struct FiveCoreGodlyBrewingTable;

struct TenCoreBasicBrewingTable;
struct TenCoreMediumBrewingTable;
struct TenCoreAdvancedBrewingTable;
struct TenCoreGodlyBrewingTable;

struct MetalBasicAnvil;
struct MetalMediumAnvil;
struct MetalAdvancedAnvil;
struct MetalGodlyAnvil;

struct RefinedBasicAnvil;
struct RefinedMediumAnvil;
struct RefinedAdvancedAnvil;
struct RefinedGodlyAnvil;

struct NobleBasicAnvil;
struct NobleMediumAnvil;
struct NobleAdvancedAnvil;
struct NobleGodlyAnvil

impl ThreeCoreBasicBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();

		let icon;
		let slot_1;
		let slot_2;
		let slot_3;
		let fuel_slot;
		let base = GridBuilder::new(WidgetBuilder::new());

		Self {
			icon
			slot_1,
			slot_2,
			slot_3,
			fuel_slot,
		}
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


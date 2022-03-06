use rg3d::engine::Engine;
use rg3d::core::Handle;
use rg3d::utils::into_gui_texture;
use rg3d::gui::{
    widget::WidgetBuilder,
    grid::{
        GridBuilder,
        Row,
        Column,
		GridDimension,
    },
    image::ImageBuilder,
    menu::{MenuItemBuilder, MenuBuilder},
    node::UiNode,
    VerticalAlignment,
    HorizontalAlignment,
    UserInterface,
	scrollbar::{ScrollBarBuilder},
	Thickness,
	VerticalAlignment
};
struct ThreeCoreBasicBrewingTable {
	icons: Vec<Handle<UiNode>>,
	fuel_slot: Handle<UiNode>,
	base: Handle<UiNode>,
	scrolling: Handle<UiNode>,
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
struct NobleGodlyAnvil;

pub struct ScrollBarData {
	pub min: f32,
	pub max: f32,
	pub value: f32,
	pub step: f32,
	pub row: usize,
	pub column: usize,
	pub margin: Thickness,
	pub show_value: bool,
	pub orientation: Orientation,
}

fn IconScrollbar(ctx: &mut BuildContext, data: ScrollBarData) -> Handle<UiNode> {
	let mut wb = WidgetBuilder::new();
	match data.orientation {
		Orientation::Vertical => wb = wb.with_width(30.0),
		Orientation::Horizontal => wb = wb.with_height(30.0),
	}
	ScrollBarBuilder::new(
		wb.on_row(data.row)
			.on_column(data.column)
			.with_margin(data.margin),
	)
	.with_min(data.min)
	.with_max(data.max)
	.with_value(data.value)
	.with_step(data.step)
	.show_value(data.show_value)
	.with_value_precision(1)
	.build(ctx);

}

impl ThreeCoreBasicBrewingTable {
    fn new(ui: &mut UserInterface) -> Handle<UiNode> {
        let ctx = &mut ui.build_ctx();

		let icons;
		let fuel_slot;
		let scrolling;
		
		GridBuilder::new(
			WidgetBuilder::new()
			.with_width(1200)
			.with_height(900)
			.with_children(
				GridBuilder::new(
					WidgetBuilder::new()
					.on_column(1)
					.on_height(1)
					.with_width(840.0)
					.with_height(1140.0)
					.with_children(
						GridBuilder:new(
							WidgetBuilder::new()
							.on_column(0)
							.on_row(0)
							.with_children(
								GridBuilder::new(
									WidgetBuilder::new()
									.on_row(0)
								)
								.add_columns(vec![
									GridDimension::strict(50.0),
									GridDimension::strict(150.0),
									GridDimension::strict(50.0)
								])
								.build(ctx),
								/// item scrolling.
								GridBuilder::new(
									WidgetBuilder::new()
									.on_row(2)
									.with_child({
										// rusty-editor scrolling we use it
										let scrolling = IconScrollbar();
									})
									.with_child(
										Handle::NONE
									)
								)
								.add_rows(vec![
									GridDimension::strict(50.0),
									GridDimension::strict(300.0),
									GridDimension::strict(50.0)
								])
								.add_columns(vec![
									GridDimension::stretch(),
									GridDimension::strict(30.0)
								])
								.build(ctx),
							)
						)
						.add_row(GridDimension::strict(250))
						.add_row(GridDimension::strict(90))
						.add_row(GridDimension::strict(800))
						.build(ctx),
						GridBuilder::new(
							WidgetBuilder::new()
							.on_column(1)
							.on_row(1)
							.with_children(
								Handle::NONE
							)
						)
						.build(ctx),
					)
					.build(ctx),
			)
			.add_column(GridDimension::strict(570))
			.add_column(GridDimension::strict(570))
			.with_opacity(25)
			.build(ctx)
		)
		)
		.add_columns(vec![
			GridDimension::strict(30),
			GridDimension::strict(1140),
			GridDimension::strict(30)
		])
		.add_rows(vec![
			GridDimension::strict(30),
			GridDimension::strict(840),
			GridDimension::strict(30)
		])
		.with_opacity(100)
		.build(ctx);

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


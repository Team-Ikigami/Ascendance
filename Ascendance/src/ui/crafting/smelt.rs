use fyrox::core::pool::Handle;
use fyrox::engine::Engine;
use fyrox::gui::{
    grid::{Column, GridBuilder, GridDimension, Row},
    image::ImageBuilder,
    menu::{MenuBuilder, MenuItemBuilder},
    UiNode,
    scroll_bar::ScrollBarBuilder,
    widget::WidgetBuilder,
	button::ButtonBuilder,
    HorizontalAlignment, Thickness, UserInterface, VerticalAlignment,
	Orientation,
	message::UiMessage,
};
use fyrox::core::algebra::Vector2;
use fyrox::utils::into_gui_texture;
use ron::de::from_reader;
use ron::value::Map;
use ron::value::Value;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, path::PathBuf, str};
/// TODO:
///
/// write an iterator that reads all the maps and creates icons for each item
/// write the base format for what is goig to be in items/brewery_possibilites.ron
/// learn how to do iterations
pub enum BTUiLevelSetter {
	Basic,
	Medium,
	Advanced,
	Godly,
}

#[derive(Deserialize, Debug)]
pub struct RonMaps {
    brewery1: Vec<HashMap<String, PathBuf>>,
}

pub struct BrewingTable {
    icons: Handle<UiNode>,
    fuel_slot: Handle<UiNode>,
    base: Handle<UiNode>,
    scrolling: Handle<UiNode>,
	level: BTUiLevelSetter,
}

pub struct Anvil;

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

fn icon_scrollbar(ui: &mut UserInterface, data: ScrollBarData) -> fyrox::core::pool::Handle<fyrox::gui::UiNode> {
	let mut ctx = ui.build_ctx();
    let mut wb = WidgetBuilder::new();
    match data.orientation {
        Orientation::Vertical => wb = wb.with_width(30.0),
        Orientation::Horizontal => wb = wb.with_height(30.0),
    }
    let scroll_bar = ScrollBarBuilder::new(
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
    .build(&mut ctx);

	scroll_bar
}

impl BrewingTable {
    pub fn new() -> Self {//Handle<UiNode>
        let ui = &mut UserInterface::new(Vector2::<f32>::new(3200.0, 3200.0));
		let ctx = &mut ui.build_ctx();

        let icons;
		GridBuilder::new(
			WidgetBuilder::default()
				.with_child({
					icons = ButtonBuilder::new(WidgetBuilder::default()).build(ctx);
					icons
				})
		).build(ctx);
        let fuel_slot;
		GridBuilder::new(
			WidgetBuilder::default()
				.with_child({
					fuel_slot = ButtonBuilder::new(WidgetBuilder::default()).build(ctx);
					fuel_slot
				})
		).build(ctx);
		let scrolling;
		GridBuilder::new(
			WidgetBuilder::default()
				.with_child({
					scrolling = ButtonBuilder::new(WidgetBuilder::default()).build(ctx);
					scrolling
				})
		).build(ctx);
		let base;
		GridBuilder::new(
			WidgetBuilder::default()
				.with_child({
					base = ButtonBuilder::new(WidgetBuilder::default()).build(ctx);
					base
				})
		).build(ctx);
//        GridBuilder::new(
//            WidgetBuilder::new()
//                .with_width(1200.0)
//                .with_height(900.0)
//                .with_child(
//                    GridBuilder::new(
//                        WidgetBuilder::new()
//                            .on_column(0)
//                            .on_row(0)
//                            .with_width(840.0)
//                            .with_height(1140.0)
//                            .with_children([
//                                GridBuilder::new(
//                                    WidgetBuilder::new().on_column(0).on_row(0).with_children([
//                                        GridBuilder::new(
//                                            WidgetBuilder::new()
//                                                .on_row(0)
//                                                .with_child(Handle::NONE),
//                                        )
//                                        .add_columns(vec![
//                                            GridDimension::strict(50.0),
//                                            GridDimension::strict(150.0),
//                                            GridDimension::strict(50.0),
//                                        ])
//                                        .build(ctx),
//                                        // item scrolling.
//                                        GridBuilder::new(
//                                            WidgetBuilder::new().on_row(2).with_children([
//                                                {
//                                                    // rusty-editor scrolling we use it
//                                                    let scrolling = IconScrollbar(ctx, ScrollBarData {
//														min: 0.0,
//														max: 0.0,
//														value: 0.0,
//														step: 1.0,
//														row: 2,
//														column: 0,
//														margin: Thickness::uniform(0.0),
//														show_value: true,
//														orientation: Orientation::Vertical,
//													});
//                                                    scrolling
//                                                },
//                                                {
//                                                    let icons = GridBuilder::new(WidgetBuilder::new());
//                                                    icons
//                                                },
//											]),
//                                        )
//                                        .add_rows(vec![
//                                            GridDimension::strict(50.0),
//                                            GridDimension::strict(300.0),
//                                            GridDimension::strict(50.0),
//                                        ])
//                                        .add_columns(vec![
//                                            GridDimension::stretch(),
//                                            GridDimension::strict(30.0),
//                                        ])
//                                        .build(ctx),
//									]),
//                                ).add_rows(vec![
//                                    GridDimension::strict(250.0),
//                                    GridDimension::strict(90.0),
//                                    GridDimension::strict(800.0),
//                                ]).build(ctx),
//                                GridBuilder::new(
//                                    WidgetBuilder::new()
//                                        .on_column(1)
//                                        .on_row(1)
//                                        .with_child(Handle::NONE),
//                                ).build(ctx),
//							]).build(ctx),
//                    )
//                    .add_columns(vec![
//                        GridDimension::strict(570.0),
//                        GridDimension::strict(570.0),
//                    ])
//                    .add_rows(vec![GridDimension::stretch()])
//                    // .with_opacity(25)
//                    .build(ctx),
//                ),
//        )
//        .add_columns(vec![
//            GridDimension::strict(30.0),
//            GridDimension::strict(1140.0),
//            GridDimension::strict(30.0),
//        ])
//        .add_rows(vec![
//            GridDimension::strict(30.0),
//            GridDimension::strict(840.0),
//            GridDimension::strict(30.0),
//        ])
//        .with_opacity(100.0)
//        .build(ctx);

        Self {
			icons,
			scrolling,
			base,
			fuel_slot,
			level: BTUiLevelSetter::Basic,
		}
    }
	pub fn handle_ui_message(&mut self, message: &UiMessage) {}
//    fn generate_icons(&mut self, ui: &mut UserInterface) -> Vec<Handle<UiNode>> {
//        let path = File::open("data/configs/brewery_possibilities.ron").unwrap();
//        let mut read: RonMaps = from_reader(path).unwrap();
//        let mut new_row = u32::new();
//        new_row = 0;
//        let mut back = items.brewery1.0.iter();
//        let number = items.len();
//        for String in items.brewery1.0.iter() {
//            icons.add_row(GridDimension::strict(20));
//            icons.with_child(
//                ButtonBuilder::new(WidgetBuilder::new().on_row(new_row))
//                    // .with_back(decorator: Handle<UiNode>)
//                    .with_text("{}", String),
//            );
//            new_row = new_row + 1;
//        }
//    }
}
// impl Anvil {
//     fn new(ui: &mut UserInterface) -> Handle<UiNode> {
//		let ctx = &mut ui.build_ctx();
// 		GridBuilder::new(WidgetBuilder::new().with_vertical_alignment(VerticalAlignment::Center).with_horizontal_alignment(HorizontalAlignment::Center))
// 		.with_child(
// 			HANDLE::None,
// 		)
// 		.with_background(
// 			ImageBuilder::new(
// 				WidgetBuilder::new()
// 					.on_row(0)
// 					.on_column(0)
// 					.with_vertical_alignment(VerticalAlignment::Center)
// 					.with_horizontal_alignment(HorizontalAlignment::Center),
// 			)
// 			.with_texture("assets/textures/icons/anvil.png")
// 			.build(ctx),
// 		)
// 		.add_columns(2)
// 		.add_rows(2)
// 		.build(ctx);
// 	}
// 	fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
// }

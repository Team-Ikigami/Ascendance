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
// functions
fn ThreeCoreBasicBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn ThreeCoreMediumBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn ThreeCoreAdvancedBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn ThreeCoreGodlyBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn FiveCoreBasicBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn FiveCoreMediumBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn FiveCoreAdvancedBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn FiveCoreGodlyBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn TenCoreBasicBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn TenCoreMediumBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn TenCoreAdvancedBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn TenCoreGodlyBrewingTable(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn MetalBasicAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn MetalMediumAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn MetalAdvancedAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
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
fn MetalGodlyAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn RefinedBasicAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn RefinedMediumAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn RefinedAdvancedAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn RefinedGodlyAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn NobleBasicAnvil(ui: &mut UserInterface) -> Handle<UiNode> {
    let ctx = &mut ui.build_ctx();
}
fn NobleMediumAnvil(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn NobleAdvancedAnvil(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn NobleGodlyAnvil(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
pub fn OpeningUI(ui: &mut UserInterface) -> Handle<UiNode> {
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

// Imports
use rg3d::{
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
    gui::{
        button::ButtonBuilder,
        widget::WidgetBuilder,
        grid::GridBuilder,
        check_box::CheckBoxBuilder,
        image::ImageBuilder,
        node::{StubNode, UiNode},
        scroll_bar::ScrollBarBuilder,
        text::TextBuilder,
        text_box::TextBoxBuilder,
        message::{
            UiMessage,
            ButtonMessage,
            CheckBoxMessage,
            ImageMessage,
            ScrollBarMessage,
            TextBoxMessage,
            MessageDirection,
            MenuMessage,
            MenuItemMessage,
            MouseButton,
            ScrollPanelMessage,
            ProgressBarMessage,
            MessageData,
            WidgetMessage
        },
        menu::{MenuBuilder, MenuItemBuilder, MenuItemContent},
        DEFAULT_FONT,
        DragContext,
        MouseState,
        Thickness,
        UserInterface
    },
    core::{
        pool::{Handle, PoolIterator, PoolIteratorMut}
    },
    asset::{
        define_new_resource, 
        Resource, 
        ResourceLoadError, 
        ResourceData, 
        ResourceState
    },
    sound::{
        source::{
            generic::GenericSourceBuilder,
            SoundSource,
            Status
        },
        context::SoundContext,
        buffer::{
            DataSource,
            SoundBufferResource
        }
    },
    utils::into_gui_texture,
};
use std::{
    borrow::Cow,
    thread,
    time::Duration
};
use serde::{Serialize, Deserialize};
use quinn::{
    ApplicationClose,
    RecvStream,
    SendStream,
}

// Structs

struct Game { }
struct Openinguibuttons {
    newgame: Handle<UiNode>,
    settings: Handle<UiNode>,
    exit: Handle<UiNode>,
}

pub struct Item {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    weight: u32,
}
struct RecipeStruct {
    name: Cow<'static, str>,
    ingredients: Vec<Ingredient>,
    result: Item,
}
struct Ingredient {
    item: Item,
    amount: u32,
}
struct CraftingMenuWidgets {

}


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
fn anvil_recipe_get(id: u32) -> RecipeStruct {}
fn cooking_recipe_get(id: u32) -> RecipeStruct {}
fn potion_recipe_get(id: u32) -> RecipeStruct {}
fn smelting_recipe_get(id: u32) -> RecipeStruct {}
pub fn OpeningUI(ui: &mut UserInterface) -> Openinguibuttons, Handle<UiNode> {
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
                    .with_items(
                        newgame = MenuItemBuilder::new(WidgetBuilder::new()
                                .on_row(1)
                                .on_column(1)
                                .with_vertical_alignment(VerticalAlignment::Center),
                        )
                        .with_content(text("New Game").shortcut("NewgameUI").icon("assets/textures/widgetbackgrounds/newgame.png"))
                        .with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
                        .build(ctx),
                        settings = MenuItemBuilder::new(
                            WidgetBuilder::new()
                                .on_row(0)
                                .on_column(0)
                                .with_vertical_alignment(VerticalAlignment::Center),
                        )
                        .with_content(text("Settings").shortcut("SettingsUI").icon("/assets/textures/icons/settings.png"))
                        .with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
                        .build(ctx),
                        exit = MenuItemBuilder::new(
                            WidgetBuilder::new()
                                .on_row(0)
                                .on_column(1)
                                .with_vertical_alignment(VerticalAlignment::Center),
                        )
                        .with_content(text("Exit").shortcut("").icon("assets/textures/icons/exitdoor.png"))
                        .with_back("assets/textures/backgrounds/itemsrectanglebackgound.png")
                        .build(ctx),
                    )
                    .build(ctx),
            )
    )
    .add_row(Row::strict(200.0))
    .add_column(Column::strict(600.0))
    .build(ctx);
}
fn Newgame() {}
fn Settings() {}
fn Exit() {}
fn Savegame() {}
fn Loadgame() {}
fn Inventory() {}
fn ServerConnect() {}
fn ServerDisconnect() {}
fn ServerSettings() {}
fn ServerStart() {}
fn ServerStop() {}
fn ServerRestart() {}
fn ServerStatus() {}
fn ServerPlayers() {}
fn ServerChat() {}
fn ServerLog() {}
fn ServerBan() {}
fn ServerKick() {}
fn ServerUnban() {}
fn ServerWhitelist() {}
fn ServerWhitelistAdd() {}
fn ServerWhitelistRemove() {}
fn ServerWhitelistList() {}
fn ServerWhitelistClear() {}
fn ServerWhitelistSave() {}
fn ServerWhitelistLoad() {}
fn ServerWhitelistImport() {}
fn ServerWhitelistExport() {}
fn ServerWhiteListImportUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhiteListExportUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhiteListClearUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhiteListSaveUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhiteListLoadUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerKickUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerBanUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerUnbanUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhitelistAddUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhitelistRemoveUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerWhitelistListUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerStatusUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerStartUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerStopUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerRestartUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerConnectUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerDisconnectUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerSettingsUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn ServerChatUI(ui: &mut UserInterface) {
    let ctx = &mut ui.build_ctx();
}
fn BasicBandit(builder: &mut ResourceManager) {
    let build = builder.request_model();
//     build("assets/model/BasicBandit.fbx")
}
fn BasicBanditWarlock(builder: &mut ResourceManager) {}
fn BasicChief(builder: &mut ResourceManager) {}
fn AverageBandit(builder: &mut ResourceManager) {}
fn AverageBanditWarlock(builder: &mut ResourceManager) {}
fn AverageBanditBarbarian(builder: &mut ResourceManager) {}
fn AverageBanditChief(builder: &mut ResourceManager) {}

// Impl

// main

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
            if message.destination() == OpeningUI.settings {
                //
                // Insert your code clicking handling code here.
                //
            }
            if message.destination() == OpeningUI.exit {
                //
                // Insert your code clicking handling code here.
                //
            }
            if message.destination() == OpeningUI.newgame {
                //
                // Insert your code clicking handling code here.
                //
            }
        }
        // if let Some(CheckBoxMessage::Check(value)) = message.data() {
        //     if message.destination() == self.checkbox {
        //         //
        //         // Insert your clicking handling code here.
        //         //
        //     }
        // }
    }
}

fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}

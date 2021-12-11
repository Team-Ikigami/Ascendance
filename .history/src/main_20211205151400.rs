// mod items;
// mod player;
// mod server;
// mod sound;
// mod ui;
// mod world;
#[ignore(unused_imports)]
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
};


// Structs

struct Game {
	newgame: Handle<UiNode>,
    settings: Handle<UiNode>,
    exit: Handle<UiNode>,
}

// fn Newgame() {}
// fn Settings() {}
// fn Exit() {}
// fn Savegame() {}
// fn Loadgame() {}
// fn Inventory() {}

// fn BasicBandit(builder: &mut ResourceManager) {
//     let build = builder.request_model();
// //     build("assets/model/BasicBandit.fbx")
// }
// fn BasicBanditWarlock(builder: &mut ResourceManager) {}
// fn BasicChief(builder: &mut ResourceManager) {}
// fn AverageBandit(builder: &mut ResourceManager) {}
// fn AverageBanditWarlock(builder: &mut ResourceManager) {}
// fn AverageBanditBarbarian(builder: &mut ResourceManager) {}
// fn AverageBanditChief(builder: &mut ResourceManager) {}

impl GameState for Game {
    fn init(engine: &mut Engine) -> Self 
        where 
            Self: Sized 
    {
        let ctx = &mut engine.user_interface.build_ctx();
        let OpeningUI = GridBuilder::new(WidgetBuilder::new())
            .with_content(
                GridBuilder::new(
                    WidgetBuilder::new()
                        .on_row(2)
                        .on_column(1),
                )
                .with_content(
                    newgame: ButtonBuilder::new(WidgetBuilder::new().on_column(2).on_row(2))
                        .with_text("New Game")
                        .build(ctx),
                    
                )
                .build(ctx),
            )
            .with_back(
                ImageBuilder::new(WidgetBuilder::new())
                    .with_texture(ctx.request_texture("data/ui/openbg2.png"))
                    .build(ctx),
            )
            .add_columns(2)
            .add_rows(2)
            .build(ctx);
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
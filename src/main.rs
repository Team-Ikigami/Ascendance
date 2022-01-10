#[allow(clippy::too_many_arguments)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]

mod items;
mod message;
mod player;
mod sound;
// mod ui;
mod world;
mod entitygen;
mod loading_screen;
mod inventory;
mod weapon;
mod door;
mod light;
mod config;
mod save_load;

use fyrox::{
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        widget::{WidgetBuilder, WidgetMessage},
        grid::{GridBuilder, GridDimension},
        check_box::CheckBoxBuilder,
        image::ImageBuilder,
        scroll_bar::ScrollBarBuilder,
        text::TextBuilder,
        text_box::{TextBoxBuilder, TextBoxMessage},
	menu::{MenuMessage, MenuItemMessage},
        message::{
            UiMessage,
            MessageDirection,
            MouseButton,
            MessageData,
        },
        menu::{MenuBuilder, MenuItemBuilder, MenuItemContent},
        DEFAULT_FONT,
        DragContext,
        MouseState,
        Thickness,
        UserInterface,
	UiNode
    },
    core::pool::{Handle, PoolIterator, PoolIteratorMut},
    asset::{
        define_new_resource, 
        Resource, 
        ResourceLoadError, 
        ResourceData, 
        ResourceState
    },
    event::{WindowEvent, DeviceId, DeviceEvent},
    utils::into_gui_texture,
    event_loop::ControlFlow,
};
use rg3d_sound::{
    source::{generic::GenericSourceBuilder, SoundSource, Status},
    engine::SoundEngine,
    context::SoundContext,
    buffer::{DataSource, SoundBufferResource},
    pool::Handle as CoreSoundHandle,
};
use std::{
    thread,
    time::Duration
};
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use rand::Rng;

// Structs

struct Game {
    bgm: Handle<UiNode>,
    voice: Handle<UiNode>,
    sfx: Handle<UiNode>,
}

// fn Newgame() {}
// fn Settings() {}
// fn Exit() {}
// fn Savegame() {}
// fn Loadgame() {}
// fn Inventory() {}

// fn BasicBandit(builder: &mut ResourceManager) {
//     let build = builder.request_model();
//     build("assets/model/BasicBandit.fbx")
// }
// fn BasicBanditWarlock(builder: &mut ResourceManager) {}
// fn BasicChief(builder: &mut ResourceManager) {}
// fn AverageBandit(builder: &mut ResourceManager) {}
// fn AverageBanditWarlock(builder: &mut ResourceManager) {}
// fn AverageBanditBarbarian(builder: &mut ResourceManager) {}
// fn AverageBanditChief(builder: &mut ResourceManager) {}
fn bgmloop() {
    let engine = SoundEngine::new();
    let context = SoundContext::new();
    engine.lock().unwrap().add_context(context.clone());
    let mut randbgmint = rand::thread_rng().gen_range(1..6);
    match randbgmint {
        1 => {
            let sound_buffer = SoundBufferResource::new_generic(rg3d_sound::futures::executor::block_on(DataSource::from_file("data/music/theme.wav")).unwrap()).unwrap();
            let source = GenericSourceBuilder::new()
                .with_buffer(sound_buffer)
                .with_status(Status::Playing)
                .build_source()
                .unwrap();
            let _source_handle: CoreSoundHandle<SoundSource> = context.state().add_source(source);
            thread::sleep(Duration::from_secs(17));
        }
        2 => bgmloop(),
        3 => bgmloop(),
        4 => bgmloop(),
        5 => bgmloop(),
        _ => bgmloop(),
    }
    bgmloop();
}
/// Uses the rg3d crates framework requirements. This section runs all the necesary functions and such.
/// init is used for initializing of the game. In here we have resoure checks, preloading of the game and other stuff.
/// on_tick is used for logic that happens every second. It has a fixed fps of 60 stored in the dt variable.
/// on_ui_message handles the messges created from interacting with the ui. if let some() statements are exetremely common here.
/// on_device_event handles the device events such as mouse and keyboard.
/// on_window_event handles the window events such as resizing.
/// on_resource_load handles the resource loading.
impl GameState for Game {
    fn init(engine: &mut Engine) -> Self 
        where 
            Self: Sized 
    {
        let ctx = &mut engine.user_interface.build_ctx();
        let soundcontenttest = SoundContext::new();
        let sound_buffer_test = SoundBufferResource::new_generic(fyrox::sound::futures::executor::block_on(DataSource::from_file("data/music/theme-test.wav")).unwrap()).unwrap();
        let sourcetest = GenericSourceBuilder::new()
            .with_buffer(sound_buffer_test)
            .with_looping(true)
            .with_status(Status::Playing)
            .build_source()
            .unwrap();
        soundcontenttest.state().add_source(sourcetest);
        thread::sleep(Duration::from_secs(1));
        
        GridBuilder::new(WidgetBuilder::new())
            .add_column(GridDimension::strict(200.0))
            .add_column(GridDimension::strict(200.0))
            .add_column(GridDimension::strict(200.0))
            .add_row(GridDimension::strict(100.0))
            .add_row(GridDimension::strict(100.0))
            .add_row(GridDimension::strict(100.0))
            .build(ctx);

        Self {
            bgm: ButtonBuilder::new(WidgetBuilder::new().on_row(2).on_column(1))
                .with_text("bgm")
                .build(ctx),
            voice: ButtonBuilder::new(WidgetBuilder::new().on_row(2).on_column(2))
                .with_text("Voice Lines")
                .build(ctx),
            sfx: ButtonBuilder::new(WidgetBuilder::new().on_row(2).on_column(3))
                .with_text("Sound Effects")
                .build(ctx),
        }
    }
	fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {}
        fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
            if let Some(ButtonMessage::Click) = message.data() {
                if message.destination == self.bgm {
                    bgmloop();
                }
                if message.destination == self.voice {
                    //
                    //  Using this to call a loop voice music function
                    //
                }
                if message.destination == self.sfx {
                    //
                    //  Using this to call a sound effects loop function
                    //
                }
            }
        }
	fn on_device_event(&mut self, engine: &mut Engine, device_id: DeviceId, event: DeviceEvent) {}
	fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent) {}
	fn on_exit(&mut self, engine: &mut Engine) {}
}

/// Uses the rg3d crate to create a window and run the game loop.
fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}

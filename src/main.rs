#![allow(clippy::too_many_arguments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(dead_code)]
#![allow(unused_imports)]

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
mod level;

use crate::{level::Level, player::Player};
use fyrox::{
    window::Fullscreen,
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
    core::{
        pool::{Handle, PoolIterator, PoolIteratorMut},
        color::Color,
        futures::executor::block_on,
    },
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
    scene::Scene,
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

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

// Structs

struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
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
            let sound_buffer = SoundBufferResource::new_generic(rg3d_sound::futures::executor::block_on(DataSource::from_file("data/music/themetest.wav"))
                .unwrap())
                .unwrap();
            
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

//        let ctx = &mut engine.user_interface.build_ctx();
//        let audioengine = SoundEngine::new();
//        let soundcontenttest = SoundContext::new();
//        audioengine.lock().unwrap().add_context(soundcontenttest.clone());
//        let sound_buffer_test = SoundBufferResource::new_generic(rg3d_sound::futures::executor::block_on(DataSource::from_file("data/music/themetest.wav")).unwrap()).unwrap();
//        let sourcetest = GenericSourceBuilder::new()
//            .with_buffer(sound_buffer_test)
//            .with_looping(true)
//            .with_status(Status::Playing)
//            .build_source()
//            .unwrap();
//        let _source_handle: CoreSoundHandle<SoundSource> = soundcontenttest.state().add_source(sourcetest);
//        thread::sleep(Duration::from_secs(17));


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
        let mut scene = Scene::new();
        scene.ambient_lighting_color = Color::opaque(150, 150, 150);
        let player = block_on(Player::new(engine.resource_manager.clone(), &mut scene));
        engine
            .get_window()
            .set_fullscreen(Some(Fullscreen::Borderless(None)))
            .set_cursor_visible(false)
            .set_cursor_grab(true)
            .set_resizable(false);
        Self {
            player,
            level: block_on(Level::new(engine.resource_manager.clone(), &mut scene)),
            scene: engine.scenes.add(scene),
        }
    }
	fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {
            let scene = &mut engine.scenes[self.scene];
            self.player.update(scene);
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {}
	fn on_device_event(
            &mut self,
            _engine: &mut Engine,
            _device_id: DeviceId,
            event: DeviceEvent
        ) {
            self.player.handle_device_event(&event);
        }
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

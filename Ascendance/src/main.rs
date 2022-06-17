#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#![doc = include_str!("../README.md")]
#![doc(html_no_source)]

// TODO: Write docs for bot/lower_body.rs
// TODO: Write docs for bot/mod.rs
// TODO: Write docs for bot/upper_body.rs
// TODO: Write docs for bot/behaviour/mod.rs
// TODO: Write docs for items/mod.rs
// TODO: Write docs for items/weapon.rs

// mod actor;
// mod bot;
// mod config;
// mod door;
// mod entitygen;
// mod inventory;
// mod items;
mod level;
// mod light;
// mod loading_screen;
// mod message;
mod player;
// mod save_load;
// mod sound;
mod ui;
// mod utils;
// use crate::items::weapon::*;
// use crate::items::definition::ItemKind;
use crate::{
    level::Level,
    player::Player
};
use clap::Parser;
use fyrox::{
    asset::{
        define_new_resource,
        Resource,
        ResourceData,
        ResourceLoadError,
        ResourceState
    },
    core::{
        color::Color,
        futures::executor::block_on,
        pool::{
            Handle,
            PoolIterator,
            PoolIteratorMut
        },
    },
    engine::{
        framework::prelude::*,
        resource_manager::ResourceManager,
        Engine
    },
    event::{
        DeviceEvent,
        DeviceId,
        WindowEvent
    },
    event_loop::ControlFlow,
    gui::{
        button::{
            ButtonBuilder,
            ButtonMessage
        },
        check_box::CheckBoxBuilder,
        grid::{
            GridBuilder,
            GridDimension
        },
        image::ImageBuilder,
        menu::{
            MenuBuilder,
            MenuItemBuilder,
            MenuItemContent
        },
        menu::{
            MenuItemMessage,
            MenuMessage
        },
        message::{
            MessageData,
            MessageDirection,
            MouseButton,
            UiMessage
        },
        scroll_bar::ScrollBarBuilder,
        text::TextBuilder,
        text_box::{
            TextBoxBuilder,
            TextBoxMessage
        },
        widget::{
            WidgetBuilder,
            WidgetMessage
        },
        DragContext,
        MouseState,
        Thickness,
        UiNode,
        UserInterface,
    },
    scene::Scene,
    utils::into_gui_texture,
    window::{
        Fullscreen,
        Icon
    },
};
use image::io::Reader as ImageReader;
use fyrox::core::rand::prelude::*;
use fyrox::core::rand::Rng;
use fyrox::core::rand::thread_rng;

// use fyrox::scene::sound::{
//     DataSource,
//     SoundBufferResource,
//     context::SoundContext,
//     SoundEngine,
//     pool::Handle as CoreSoundHandle,
//     Sound,
//     Status,
// };
use serde::{
    Deserialize,
    Serialize
};
use std::{
    thread,
    time::Duration
};
use crate::ui::craftingui::BrewingTable;

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

// Structs

struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
	brewtable_ui: BrewingTable,
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

// fn bgmloop() {
//     let engine = SoundEngine::new();
//     let context = SoundContext::new();
//     engine.lock().unwrap().add_context(context.clone());
//     let mut randbgmint: u8 = rand::thread_rng().gen_range(1..6);
//     match randbgmint {
//         1 => {
//             let sound_buffer = SoundBufferResource::new_generic(
//                 fyrox::scene::sound::futures::executor::block_on(DataSource::from_file(
//                     "data/music/themetest.wav",
//                 ))
//                 .unwrap(),
//             )
//             .unwrap();
// 
//             let source = SoundBuilder::new()
// 				.with_spatial_blend_factor(0.0)
// 				.with_buffer(sound_buffer)
//                 .with_status(Status::Playing)
//                 .build_source()
//                 .unwrap();
//             let _source_handle: CoreSoundHandle<SoundSource> = context.state().add_source(source);
// 
//             thread::sleep(Duration::from_secs(17));
//         }
//         2 => bgmloop(),
//         3 => bgmloop(),
//         4 => bgmloop(),
//         5 => bgmloop(),
//         _ => bgmloop(),
//     }
//     bgmloop();
// }

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
        Self: Sized,
    {
        let mut scene = Scene::new();
        scene.ambient_lighting_color = Color::opaque(150, 150, 150);
        let player = block_on(Player::new(engine.resource_manager.clone(), &mut scene));
        let img = ImageReader::open("data/textures/icons/window_icon.png")
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let mut pixels = Vec::new();
        img.pixels().for_each(|pixel| {
            pixel.0.iter().for_each(|byte| {
                pixels.push(*byte);
            })
        });
        engine
            .get_window()
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
        engine.get_window().set_window_icon(Some(
            Icon::from_rgba(pixels, img.width(), img.height()).unwrap(),
        ));
        engine.get_window().set_cursor_visible(false);
		engine.get_window().set_cursor_grab(true).expect("Damn bro, no cursors?");
        engine.get_window().set_resizable(false);
		
		let brewtable_ui = BrewingTable::new();
		
        Self {
            player,
            level: block_on(Level::new(engine.resource_manager.clone(), &mut scene)),
            scene: engine.scenes.add(scene),
			brewtable_ui,
        }
    }
    fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {
        let scene = &mut engine.scenes[self.scene];
        self.player.update(scene);
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
		self.brewtable_ui.handle_ui_message(&message);

	}
    fn on_device_event(&mut self, _engine: &mut Engine, _device_id: DeviceId, event: DeviceEvent) {
        self.player.handle_device_event(&event);
    }
    fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent) {
		match event {
			_ => {}
		}
	}
    fn on_exit(&mut self, engine: &mut Engine) {}
}

/// Uses the rg3d crate to create a window and run the game loop.
 fn main() {
	let args = Cli::parse();
	let start_game = args.start_game;
	std::thread::spawn(move || {
		loop {
			if start_game == "True".to_string() {
				println!("what the fuck, the game already runs. What the hell you want me to do?!")
			} else if start_game == "False".to_string() {
				println!("Not starting game, what do you want to do?");
			}
			else {
				println!("INVALID ARGUMENTS");
			}
		}
	});
	Framework::<Game>::new().unwrap().title("FUCKING FINALLY").run();
}
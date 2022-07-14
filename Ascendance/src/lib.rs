#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_patterns)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#![doc = include_str!("../README.md")]
#![doc(html_no_source)]

//! TODO: Write docs for bot/lower_body.rs
//! TODO: Write docs for bot/mod.rs
//! TODO: Write docs for bot/upper_body.rs
//! TODO: Write docs for bot/behaviour/mod.rs
//! TODO: Write docs for items/mod.rs
//! TODO: Write docs for items/weapon.rs
//! Game project.

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
use std::sync::Arc;
use fyrox::engine::SerializationContext;
use fyrox::gui::BuildContext;
use fyrox::{
    core::{uuid::{uuid, Uuid}},
    event::Event,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use crate::{
    level::Level,
    player::Player
};
use fyrox::event_loop::EventLoop;
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
        pool::{Handle, PoolIterator, PoolIteratorMut},
    },
    engine::{framework::prelude::*, resource_manager::ResourceManager, Engine},
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ControlFlow,
    gui::{
        core::algebra::{Vector2},
        button::{ButtonBuilder, ButtonMessage},
        check_box::CheckBoxBuilder,
        grid::{GridBuilder, GridDimension},
        image::ImageBuilder,
        menu::{MenuBuilder, MenuItemBuilder, MenuItemContent},
        menu::{MenuItemMessage, MenuMessage},
        message::{
            MessageData,
            MessageDirection,
            MouseButton,
            UiMessage
        },
        scroll_bar::ScrollBarBuilder,
        text::TextBuilder,
        text_box::{TextBoxBuilder, TextBoxMessage},
        widget::{WidgetBuilder, WidgetMessage},
        DragContext,
        MouseState,
        Thickness,
        UiNode,
        UserInterface,
    },
    scene::{
		base::BaseBuilder,
		sound::{SoundBuilder, Status},
	},
    utils::into_gui_texture,
    window::{Fullscreen, Icon},
};
use image::io::Reader as ImageReader;
use fyrox::core::rand::prelude::*;
use fyrox::core::rand::Rng;
use fyrox::core::rand::thread_rng;
use fyrox::engine::EngineInitParams;
use crate::ui::game::start_menu_ui::StartGame;
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
// use crate::ui::crafting::smelt::BrewingTable;

use git_version::git_version;
const GIT_VERSION: &str = git_version!();

pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            scene: Default::default(),
        }
    }

    fn set_scene(&mut self, scene: Handle<Scene>, _context: PluginContext) {
        self.scene = scene;

        // Do additional actions with scene here.
    }
}

impl Plugin for Game {
    fn on_register(&mut self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn on_standalone_init(&mut self, context: PluginContext) {
        self.set_scene(context.scenes.add(Scene::new()), context);
    }

    fn on_enter_play_mode(&mut self, scene: Handle<Scene>, context: PluginContext) {
        // Obtain scene from the editor.
        self.set_scene(scene, context);
    }

    fn on_leave_play_mode(&mut self, context: PluginContext) {
        self.set_scene(Handle::NONE, context)
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn id(&self) -> Uuid {
        // Ideally this should be unique per-project.
        uuid!("cb358b1c-fc23-4c44-9e59-0a9671324196")
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_unload(&mut self, _context: &mut PluginContext) {
        // Do a cleanup here.
    }
}

struct Gamiwiie {
    bgm_button: Handle<UiNode>,
    scene: Handle<Scene>,
    level: Level,
    player: Player,
    // brewtable_ui: BrewingTable,
	start_game: Handle<UiNode>,
    exit: Handle<UiNode>,
    base_grid: Handle<UiNode>,
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

/// Uses the rg3d crates framework requirements. This section runs all the necesary functions and such.
/// init is used for initializing of the game. In here we have resoure checks, preloading of the game and other stuff.
/// on_tick is used for logic that happens every second. It has a fixed fps of 60 stored in the dt variable.
/// on_ui_message handles the messges created from interacting with the ui. if let some() statements are exetremely common here.
/// on_device_event handles the device events such as mouse and keyboard.
/// on_window_event handles the window events such as resizing.
/// on_resource_load handles the resource loading.
impl GameState for Gamiwiie {
    fn init(engine: &mut Engine) -> Self
    where
        Self: Sized,
    {
        let mut scene = Scene::new();
        scene.ambient_lighting_color = Color::opaque(150, 150, 150);
        let player = block_on(Player::new(engine.resource_manager.clone(), &mut scene));
        // let mut scene_init = Scene::new();
        // let scene = engine.scenes.add(scene_init);

        let bgm_button;
		let start_game;
        let exit;
        let base_grid = GridBuilder::new(
            WidgetBuilder::new()
            .with_height(800.0)
            .with_width(800.0)
            .with_draw_on_top(true)
            .with_children([
                {
                    start_game = ButtonBuilder::new(
                        WidgetBuilder::new()
                        .on_column(1)
                        .on_row(1)
                        .with_width(200.0)
                        .with_height(75.0)
                        )
                        .with_text("Start game")
                        .build(&mut engine.user_interface.build_ctx());
                    start_game
                },
                {
                    exit = ButtonBuilder::new(
                        WidgetBuilder::new()
                        .on_column(1)
                        .on_row(5)
                        .with_width(200.0)
                        .with_height(75.0)
                        )
                        .with_text("Exit")
                        .build(&mut engine.user_interface.build_ctx());
                    exit

                },
				{
					bgm_button = ButtonBuilder::new(
						WidgetBuilder::new()
							.on_row(3)
							.on_column(1)
							.with_width(200.0)
							.with_height(75.0)
					)
					.with_text("Play BGM")
					.build(&mut engine.user_interface.build_ctx());
					bgm_button
				}
            ])
        )
        .add_column(GridDimension::strict(100.0))
        .add_column(GridDimension::strict(200.0))
        .add_column(GridDimension::strict(500.0))
        .add_row(GridDimension::strict(300.0))
        .add_row(GridDimension::strict(75.0))
        .add_row(GridDimension::strict(92.0))
        .add_row(GridDimension::strict(75.0))
        .add_row(GridDimension::strict(92.0))
        .add_row(GridDimension::strict(75.0))
        .add_row(GridDimension::strict(92.0))
        .build(&mut engine.user_interface.build_ctx());

        // let img = ImageReader::open("data/textures/icons/window_icon.png")
        //     .unwrap()
        //     .with_guessed_format()
        //     .unwrap()
        //     .decode()
        //     .unwrap()
        //     .into_rgba8();
        // let mut pixels = Vec::new();
        // img.pixels().for_each(|pixel| {
        //     pixel.0.iter().for_each(|byte| {
        //         pixels.push(*byte);
        //     })
        // });
        engine
            .get_window()
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
        // engine.get_window().set_window_icon(Some(
        //     Icon::from_rgba(pixels, img.width(), img.height()).unwrap(),
        // ));
        // engine.get_window().set_cursor_visible(false);
	//	engine.get_window().set_cursor_grab(true).expect("Damn bro, no cursors?");
        // engine.get_window().set_resizable(false);	
	// let brewtable_ui = BrewingTable::new();
		
        Self {
            player,
            level: block_on(Level::new(engine.resource_manager.clone(), &mut scene)),
            scene: engine.scenes.add(scene),
	    // brewtable_ui,
            bgm_button,
            // scene,
			base_grid,
			exit,
			start_game,
        }
    }
    fn on_tick(&mut self, engine: &mut Engine, dt: f32, control_flow: &mut ControlFlow) {
        // let scene = &mut engine.scenes[self.scene];
        // self.player.update(scene);
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
	// self.brewtable_ui.handle_ui_message(&message);
        // self.start_game.handle_ui_message(&message, engine);
        let scene = &mut engine.scenes[self.scene];
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination == self.bgm_button {
				let mut randbgmint: i8 = 1;
                match randbgmint {
    		    	1 => {
						let sound = block_on(engine.resource_manager.request_sound_buffer("data/music/church/holy-cathedral-wav.wav")).unwrap();
						let sound_handle = SoundBuilder::new(BaseBuilder::new())
						    .with_buffer(Some(sound))
						    .with_status(Status::Playing)
						    .with_looping(true)
						    .build(&mut scene.graph);
                    }
    		    	_ => println!("Damn i guess i dont know random number generators well src/main.rs L144"),
                }
            }
			else if message.destination == self.start_game {
				let start_game = std::thread::spawn(|| {
					// start_game();
					println!("START_GAME BUTCH");
				});
			}
        }

	}
    fn on_device_event(&mut self, _engine: &mut Engine, _device_id: DeviceId, event: DeviceEvent) {
        // self.player.handle_device_event(&event);
    }
    fn on_window_event(&mut self, engine: &mut Engine, event: WindowEvent) {
		match event {
			_ => {}
		}
	}
    fn on_exit(&mut self, engine: &mut Engine) {}
}

#[derive(Parser)]
#[derive(Debug)]
struct Cli {
       #[clap(short='a', long="add-to-inventory")]
       add_to_inventory: Option<Vec<String>>,
       // add_to_inventory: std::string::ToString<ItemKind>,
       #[clap(short, long)]
       set_user_health: Option<Vec<u32>>,
       #[clap(short, long)]
       set_bot_health: u32,
       #[clap(short='n', long="start-game")]
       start_game: String,
}

// Starting from here on will be transmuted code.

struct LoadingScreen {
	root: Handle<UiNode>,
	progress_bar: Handle<UiNode>,
}
impl LoadingScreen {
	fn new(ctx: &mut BuildContext, width: f32, height: f32,) -> Self {
		let progress_bar;
		let root = GridBuilder::new(
			WidgetBuilder::new()
			.with_child(
				GridBuilder::new(
					WidgetBuilder::new()
					.with_child({
						// I am aware this is a textbuilder, i just need to pass compilation for rustdoc
						progress_bar = TextBuilder::new(
							WidgetBuilder::new()
						)
						.build(ctx);
						progress_bar
					})
				)
				.build(ctx),
			)
		)
		.build(ctx);
		Self { root, progress_bar }
	}
}

struct GameMANUALENGINEINIT {}
impl GameMANUALENGINEINIT {
	pub fn run() {
		let events_loop = EventLoop::<()>::new();

		let primary_monitor = events_loop.primary_monitor().unwrap();
        let mut monitor_dimensions = primary_monitor.size();
        monitor_dimensions.height = (monitor_dimensions.height as f32 * 0.7) as u32;
        monitor_dimensions.width = (monitor_dimensions.width as f32 * 0.7) as u32;
        let inner_size = monitor_dimensions.to_logical::<f32>(primary_monitor.scale_factor());

		let window_builder = fyrox::window::WindowBuilder::new().with_title("Ascendance")
			.with_resizable(false)
			.with_inner_size(inner_size)
			.with_always_on_top(true)
			.with_fullscreen(Some(Fullscreen::Borderless(None)));
		let serialization_context = Arc::new(SerializationContext::new());
		let mut engine = Engine::new(EngineInitParams {
		    window_builder,
		    resource_manager: ResourceManager::new(serialization_context.clone()),
		    serialization_context,
		    events_loop: &events_loop,
		    vsync: false,
		})
		.unwrap();
		let fixed_timestep = 1.0 / FIXED_FPS;
	}
}
pub const FIXED_FPS: f32 = 60.0;
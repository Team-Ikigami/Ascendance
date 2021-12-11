// mod items;
// mod player;
// mod server;
// mod sound;
// mod ui;
// mod world;
use rg3d::{
    engine::{
        Engine,
        framework::prelude::*,
        resource_manager::ResourceManager
    },
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        widget::{WidgetBuilder, WidgetMessage},
        grid::GridBuilder,
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
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


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
//     build("assets/model/BasicBandit.fbx")
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
        GridBuilder::new(WidgetBuilder::new()).add_columns(2).add_rows(3).build(ctx);
        Self {
			emailbox: TextBoxBuilder::new(WidgetBuilder::new().on_row(1).on_column(1));
			emailbutton: ButtonBuilder::new(WidgetBuilder::new().on_row(2).on_column(1));
		}
    }
    fn on_ui_message(&mut self, engine: &mut Engine, message: UiMessage) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == Self.emailbutton {
				let toperson = Self.emailbox.text();
                let email = Message::builder()
					.from("NoBody <nobody@domain.tld>".parse().unwrap())
					.reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
					.to("Hei <hei@domain.tld>".parse().unwrap())
					.subject("Happy new year")
					.body(String::from("Be happy!"))
					.unwrap();

				let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

				// Open a remote connection to gmail
				let mailer = SmtpTransport::relay("smtp.gmail.com")
					.unwrap()
					.credentials(creds)
					.build();

				// Send the email
				match mailer.send(&email) {
					Ok(_) => println!("Email sent successfully!"),
					Err(e) => panic!("Could not send email: {:?}", e),
				}
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

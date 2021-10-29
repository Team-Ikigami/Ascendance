// #[allow(unused_imports)]
// mod items;
// mod user;
// mod user_interfaces;
// mod statistics;

// use items::{
//     item::{
//         weapons::{
//             magic::{},
//             melee::{},
//             ranged::{},
//         },
//         miscellaneous::{},
//         construction::{},
//         clothing::{},
//     },
// };
// use user::{
//     user::{save_player, load_player},
// };
// use user_interfaces::{
//     crafting::{
//         advancedanvil::{layout}
//     }
// }
// use savefile::*;
// use ron::*;
use crate::shared::create_camera;
use rg3d::engine::resource_manager::MaterialSearchOptions;
use rg3d::engine::Engine;
use rg3d::gui::UiNode;
use rg3d::utils::log::{Log, MessageKind};
use rg3d::{
    animation::Animation,
    core::{
        algebra::{UnitQuaternion, Vector2, Vector3},
        color::Color,
        pool::Handle,
    },
    engine::resource_manager::ResourceManager,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        border::BorderBuilder,
        button::ButtonBuilder,
        decorator::DecoratorBuilder,
        dropdown_list::DropdownListBuilder,
        grid::{Column, GridBuilder, Row},
        message::{
            ButtonMessage, DropdownListMessage, MessageDirection, ScrollBarMessage, TextMessage,
            UiMessageData,
        },
        scroll_bar::ScrollBarBuilder,
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        HorizontalAlignment, Orientation, Thickness, VerticalAlignment,
    },
    monitor::VideoMode,
    scene::{node::Node, Scene},
    utils::translate_event,
    window::Fullscreen,
};
use rg3d_ui::{
    formatted_text::{WrapMode},
    window::{
        WindowBuilder
    },
    message::{
        MessageBox,
        ScrollBarMessage,
        TextMessage,
    },
};
use std::time::Instant;

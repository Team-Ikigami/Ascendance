use fyrox::{
	core::pool::Pool,
    engine::Engine,
	core::pool::Handle,
	gui::DragContext,
	gui::MouseState,
	gui::NodeHandleMapping,
	gui::RestrictionEntry,
	gui::Thickness,
	gui::UiNode,
	gui::UserInterface,
	gui::button::Button,
	gui::button::ButtonBuilder,
	gui::button::ButtonMessage,
	gui::grid::Grid,
	gui::grid::GridBuilder,
	gui::grid::GridDimension,
	gui::message::KeyboardModifiers,
	gui::message::UiMessage,
	gui::message::UserMessageData,
	gui::widget::Widget,
	gui::widget::WidgetBuilder,
};
use std::thread::Thread;

pub struct StartGame {
    start_game: Handle<UiNode>,
    exit: Handle<UiNode>,
    base_grid: Handle<UiNode>,
}
impl StartGame {
    pub fn new(ui: &mut UserInterface) -> Self
	where
		Self: Sized,
	{

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
                        .build(&mut ui.build_ctx());
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
                        .build(&mut ui.build_ctx());
                    exit

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
        .build(&mut ui.build_ctx());

        Self {
            start_game,
            exit,
            base_grid,
        }
	}
    pub fn handle_ui_message(&mut self, message: &UiMessage, engine: &mut Engine) {
		let mut ctx = engine.user_interface.build_ctx();
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination == self.start_game {
                let generate_world = std::thread::spawn(|| {
                    // crate::start_game::start();
                    println!("STARTING_GAME");
				});
                let mut pool = Pool::<UiNode>::new();
                pool.free(self.base_grid);
                assert_eq!(self.base_grid, Handle::NONE);
                let base_grid = GridBuilder::new(WidgetBuilder::new()).build(&mut ctx);
                self.base_grid = base_grid;
                generate_world.join().unwrap();
            }
        }   
    }
}

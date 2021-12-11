pub fn Opening() {
    GridBuilder::new(
        WidgetBuilder::new()
            .with_back(
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(1)
                        .on_column(1),
                )
                .with_texture(into_gui_texture(resource_manager.request_texture("../../../assets/misc/opening.gif")))
                .build(ctx),
            )
            .with_child(
                MenuBuilder::new(
                    .with_items(
                        MenuItemBuilder::new(
                            .with_content(
                                .text("New Game")
                                .shortcut("")
                                .icon("../../../../assets/textures/icons/Savegameicon.png")
                            )
                            .with_back("../../../../assets/textures/backgrounds/itemsrectanglebackgound.png")
                        )
                        MenuItemBuilder::new(
                            .with_content(
                                .text("Exit")
                                .shortcut("newgame")
                                .icon("../../../../assets/textures/icons/Loadgameicon.png")
                            )
                            .with_back("../../../../assets/textures/backgrounds/itemsrectanglebackgound.png")
                        )
                        MenuItemBuilder::new(
                            .with_content(
                                .text("Settings")
                                .shortcut("")
                                .icon("../../../../assets/textures/icons/exitdoor.png")
                            )
                            .with_back("../../../../assets/textures/backgrounds/itemsrectanglebackgound.png")
                        )
                        MenuItemBuilder::new(
                            .with_content(
                                .text("Exit")
                                .shortcut("")
                                .icon("../../../../assets/textures/icons/exitdoor.png")
                            )
                            .with_back("../../../../assets/textures/backgrounds/itemsrectanglebackgound.png")
                        )
                        .build(ctx),
                    )
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
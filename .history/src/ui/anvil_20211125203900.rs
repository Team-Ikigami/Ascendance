fn MetalBasicAnvil() {}
fn MetalMediumAnvil() {}
fn MetalAdvancedAnvil() {
    let ctx = &mut engine.user_interface.build_ctx();
    let scrollbar;
    GridBuilder::new(
        WidgetBuilder::new()
            .with_child(
                ImageBuilder::new(
                    WidgetBuilder::new()
                        .on_row(0)
             
                        .on_column(0)
                        .with_vertical_alignment(VerticalAlignment::Center),
                )
            .with_texture("../../../assets/textures/icons/anvil.png")
            .build(ctx)
        )
        .with_child({
            scrollbar = ScrollBarBuilder::new(
                WidgetBuilder::new()
                    .on_row(0)
                    .on_column(1)
                    .with_vertical_alignment(VerticalAlignment::Center)
                    .with_margin(Thickness::uniform(2.0)),
                    .with_texture("../../../assets/textures/icons/scrollbar.png")
            )
            .with_min(0.0)
            .with_max(360.0)
            .with_value(DEFAULT_MODEL_ROTATION)
            .with_step(5.0)
            .show_value(true)
            .with_value_precision(0)
            .build(ctx);
            scrollbar
        })
        .with_child(
            TextBuilder::new(
                WidgetBuilder::new()
                    .on_row(1)
                    .on_column(0)
                    .with_vertical_alignment(VerticalAlignment::Center),
            )
            .with_wrap(WrapMode::Word)
            .with_text("Scale")
            .build(ctx),
        )
    )
}
fn MetalGodlyAnvil() {}
fn RefinedBasicAnvil() {}
fn RefinedMediumAnvil() {}
fn RefinedAdvancedAnvil() {}
fn RefinedGodlyAnvil() {}
fn NobleBasicAnvil() {}
fn NobleMediumAnvil() {}
fn NobleAdvancedAnvil() {}
fn NobleGodlyAnvil() {}
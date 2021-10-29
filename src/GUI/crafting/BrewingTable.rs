fn ThreeCoreBasicBrewingTable() {
    println!("f");
}
fn ThreeCoreMediumBrewingTable() {
    println!("Three core brewing stand");
}
fn ThreeCoreAdvancedBrewingTable() {
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
            ButtonBuilder::new(
                .with_child(
                    ImageBuilder::new(
                        WidgetBuilder::new()
                            .on_row(1)
                            .on_column(0)
                            .with_vertical_alignment(VerticalAlignment::Center),
                    )
                .with_texture("../../../assets/shared/icons/exitui.png")
                .build(ctx)
                )
            )
        })
    )   
}
fn ThreeCoreGodlyBrewingTable() {
    println!("Three core brewing stand");
}
fn FiveCoreBasicBrewingTable() {
    println!("f");
}
fn FiveCoreMediumBrewingTable() {
    println!("Three core brewing stand");
}
fn FiveCoreAdvancedBrewingTable() {
    let ctx = &mut engine.user_interface.build_ctx();
    let scrollbar;
    println!("Three core brewing stand");
}
fn FiveCoreGodlyBrewingTable() {
    println!("Five core");
}
fn TenCoreBasicBrewingTable() {
    println!("f");
}
fn TenCoreMediumBrewingTable() {
    println!("brubfn");
}
fn TenCoreAdvancedBrewingTable() {
    let ctx = &mut engine.user_interface.build_ctx();
    let scrollbar;
    println!("Three core brewing stand");
}
fn TenCoreGodlyBrewingTable() {
    println!("Five core");
}
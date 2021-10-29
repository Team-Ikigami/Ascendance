// use rg3d_ui::*;

struct threecore;
struct fivecore;
struct tencore;
pub impl threecore {
    struct interface;
    pub mod interface {
        pub fn dunno() {
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
        .with_child
    }
    fn tasks() {
        println!("doing tasks rgsrcuicraftadvbretabthrecortas")
    }
}
pub impl fivecore {
    struct interface;
    pub impl interface {
        pub fn dunno() {
            let ctx = &mut engine.user_interface.build_ctx();
            let scrollbar;
            println!("Three core brewing stand");
        }
    }
    fn tasks() {
        println!("doing tasks rgsrcuicraftadvbretabfivcortas")
    }
}
pub impl tencore{
    struct interface;
    pub mod interface {
        pub fn dunno() {
            let ctx = &mut engine.user_interface.build_ctx();
            let scrollbar;
            println!("Three core brewing stand");
        }
    }
    fn tasks() {

    }
}
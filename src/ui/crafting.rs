mod super::other::Recipe;

use Recipe::{
    anvil_recipe_get,
    potion_recipe_get,
    smelting_recipe_get,
    cooking_recipe_get,
};
use rg3d_ui::{
    grid::{GridBuilder, Grid},
    widget::{WidgetBuilder, Widget}
    Button::{ButtonBuilder, Button},
    image::{ImageBuilder, Image},
    border::{BorderBuilder, Border},
    text::{TextBuilder, Text},
    node::{UINode, StubNode},
}


struct CraftingMenuWidgets {
        
}

fn furnace_learn_recipe() -> CraftingMenuWidgets {
    let LearnSmelting = GridBuilder::new(
        .with_content()
    )
    .columns(2)
    .rows(2)
    .build();
}

fn cooking_menu_list() -> CraftingMenuWidgets {
    let ThreeCoreAdvancedBrewingTable = GridBuilder::new(
        .with_content()
    )
    .columns(2)
    .rows(2)
    .build();

}

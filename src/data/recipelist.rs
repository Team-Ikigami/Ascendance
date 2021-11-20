mod super::other;

use other::{
    Recipe::{
        RecipeStruct,
        Ingredient,
    },
    item::{
        RUSTY_SWORD,
        RUSTY_SHIELD,
        HEALTH_POTION,
        MANA_POTION,
        STEEL_SWORD,
    },
};

const FRIED_FOWL_EGGS: RecipeStruct = RecipeStruct {
    name: "Fried Fowl Eggs",
    ingredients: vec![
        Ingredient {
            item: FOWL_EGGS,
            amount: 2,
        },
    ],
    result: Item::FriedFowlEggs,
};
const RUSTY_SWORD: RecipeStruct = RecipeStruct {
    name: "Rusty Sword",
    ingredients: vec![
        Ingredient {
            item: IRON_INGOT,
            amount: 5,
        },
    ],
    result: RUSTY_SWORD,
};
// Recipe List

const FRIED_FOWL_EGGS_RECIPE: RecipeStruct = RecipeStruct {
    name: Cow::Borrowed("Fried Fowl Eggs"),
    ingredients: vec![
        Ingredient {
            item: FOWL_EGG,
            amount: 2,
        },
    ],
    result: FRIED_FOWL_EGGS,
};
const RUSTY_SWORD_RECIPE: RecipeStruct = RecipeStruct {
    name: Cow::Borrowed("Rusty Sword"),
    ingredients: vec![
        Ingredient {
            item: IRON_INGOT,
            amount: 5,
        },
    ],
    result: RUSTY_SWORD,
};
const RUSTY_SHIELD_RECIPE: RecipeStruct = RecipeStruct {
    name: Cow::Borrowed("Rusty Shield"),
    ingredients: vec![
        Ingredient {
            item: IRON_INGOT,
            amount: 5,
        },
    ],
    result: RUSTY_IRON_SHIELD,
};
const RUSTY_IRON_HELMET_RECIPE: RecipeStruct = RecipeStruct {
    name: Cow::Borrowed("Rusty Helmet"),
    ingredients: vec![
        Ingredient {
            item: IRON_INGOT,
            amount: 5,
        },
    ],
    result: RUSTY_IRON_HELMET,
};
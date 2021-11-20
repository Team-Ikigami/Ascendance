mod item;
use std::borrow::Cow;
use item::*;
use item::Item;
struct RecipeStruct {
    name: Cow<'static, str>,
    ingredients: Vec<Ingredient>,
    result: Item,
}
struct Ingredient {
    item: Item,
    amount: u32,
}


fn anvil_recipe_get(id: u32) -> RecipeStruct {
    println!("No");
}

fn cooking_recipe_get(id: u32) -> RecipeStruct {
    let recipe = match id {
        1 => {
            let mut recipe = Recipe {
                name: "Fried Fowl Eggs".to_string(),
                ingredients: Vec::new(),
                result: item::FRIED_FOWL_EGG,
            };
            recipe.ingredients.push(Ingredient {
                item: Item {
                    name: "Fowl Egg".to_string(),
                    description: "An egg of the common fowl.".to_string(),
                    weight: 1,
                },
                amount: 1,
            });
            recipe
        }
        _ => panic!("Invalid recipe id"),
    };
    recipe
}
fn potion_recipe_get(id: u32) -> RecipeStruct {
    let recipe = match id {
        1 => {
            let mut recipe = Recipe {
                name: "Rusty sword".to_string(),
                ingredients: Vec::new(),
                result: Item {
                    name: "Rusty sword".to_string(),
                    description: "A rusty sword.".to_string(),
                    weight: 1,
                },
            };
            recipe.ingredients.push(Ingredient {
                item: Item {
                    name: "Rusty sword".to_string(),
                    description: "A rusty sword.".to_string(),
                    weight: 1,
                },
                amount: 1,
            });
            recipe
        }
        _ => panic!("Invalid recipe id"),
    };
    recipe
}
fn smelting_recipe_get(id: u32) -> RecipeStruct {
    let recipe = match id {
        1 => {
            let mut recipe = Recipe {
                name: "Rusty sword".to_string(),
                ingredients: Vec::new(),
                result: Item {
                    name: "Rusty sword".to_string(),
                    description: "A rusty sword.".to_string(),
                    weight: 1,
                },
            };
            recipe.ingredients.push(Ingredient {
                item: Item {
                    name: "Rusty sword".to_string(),
                    description: "A rusty sword.".to_string(),
                    weight: 1,
                },
                amount: 1,
            });
            recipe
        }
        _ => panic!("Invalid recipe id"),
    };
    recipe
}
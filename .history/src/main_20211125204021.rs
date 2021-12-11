use rg3d::{
    engine::Engine,
    engine::framework::prelude::*,
};
use std::borrow::Cow;
// Structs

struct Game { }

pub struct Item {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    weight: u32,
}
struct RecipeStruct {
    name: Cow<'static, str>,
    ingredients: Vec<Ingredient>,
    result: Item,
}
struct Ingredient {
    item: Item,
    amount: u32,
}
struct CraftingMenuWidgets {

}

// Constants
// Recipe List

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

// weapon and armor constants
pub const RUSTY_SWORD: Item = Item {
    name: Cow::Borrowed("Rusty Sword"),
    description: Cow::Borrowed("A rusty sword."),
    weight: 5,
};
pub const FOWL_EGG: Item = Item {
    name: Cow::Borrowed("Fowl Egg"),
    description: Cow::Borrowed("A fowl egg."),
    weight: 1,
};
pub const FRIED_FOWL_EGGS: Item = Item {
    name: Cow::Borrowed("Fried Fowl Egg"),
    description: Cow::Borrowed("A fried fowl egg. Good for a quick snack and tastes best with salt."),
    weight: 1,
}
pub const RUSTY_KEY: Item = Item {
    name: Cow::Borrowed("Rusty Key"),
    description: Cow::Borrowed("A rusty key. Unsure what it unlocks."),
    weight: 1,
};
pub const RUSTY_PICK: Item = Item {
    name: Cow::Borrowed("Rusty Pick"),
    description: Cow::Borrowed("A rusty pick. Will break inside any lock you put it in."),
    weight: 1,
};
pub const RUSTY_SHOVEL: Item = Item {
    name: Cow::Borrowed("Rusty Shovel"),
    description: Cow::Borrowed("A rusty shovel. It's strangely durable"),
    weight: 1,
};
pub const RUSTY_AXE: Item = Item {
    name: Cow::Borrowed("Rusty Axe"),
    description: Cow::Borrowed("A rusty axe. It's strangely durable"),
    weight: 1,
};
pub const SPECIAL_RUSTY_KEY: Item = Item {
    name: Cow::Borrowed("Special Rusty Key"),
    description: Cow::Borrowed("A special rusty key. It's a key that unlocks something special."),
    weight: 1,
};
pub const DRAGON_SCALE: Item = Item {
    name: Cow::Borrowed("Dragon Scale"),
    description: Cow::Borrowed("A dragon scale. It's a bit of a mystery."),
    weight: 3,
};
pub const DRAGON_SCALE_ARMOR: Item = Item {
    name: Cow::Borrowed("Dragon Scale Armor"),
    description: Cow::Borrowed("A dragon scale armor. Forged from the scales of the strongest beast, it's a reminder of our accomplishments."),
    weight: 50,
};
pub const DRAGON_SCALE_SHIELD: Item = Item {
    name: Cow::Borrowed("Dragon Scale Shield"),
    description: Cow::Borrowed("A dragon scale shield. Forged from the scales of the strongest beast, will block any fire attack up to rank 30 and any magic attack up to rank 5."),
    weight: 20,
};
pub const DRAGON_SCALE_HELMET: Item = Item {
    name: Cow::Borrowed("Dragon Scale Helmet"),
    description: Cow::Borrowed("A dragon scale helmet. Forged from the scales of the strongest beast, it's a reminder of our accomplishments."),
    weight: 5,
};
pub const DRAGON_SCALE_ARMOR_PLATE: Item = Item {
    name: Cow::Borrowed("Dragon Scale Armor Plate"),
    description: Cow::Borrowed("A dragon scale armor plate. Forged from the scales of the strongest beast, it's a reminder of our accomplishments."),
    weight: 50,
};
pub const DRAGON_BONE: Item = Item {
    name: Cow::Borrowed("Dragon bone"),
    description: Cow::Borrowed("The bones of the legendary beasts of the skies."),
    weight: 5,
};
pub const DRAGON_BONE_ARMOR: Item = Item {
    name: Cow::Borrowed("Dragon bone armor"),
    description: Cow::Borrowed("The bones of the dragon being used as a defense item brings out great strength in even the weakest of warriors"),
    weight: 50,
};
pub const IRON_INGOT: Item = Item {
    name: Cow::Borrowed("Iron Ingot"),
    description: Cow::Borrowed("An iron ingot. It's a bit of a mystery."),
    weight: 1,
};
pub const IRON_SWORD: Item = Item {
    name: Cow::Borrowed("Iron Sword"),
    description: Cow::Borrowed("An iron sword. It's a bit of a mystery."),
    weight: 5,
};
pub const IRON_SHOVEL: Item = Item {
    name: Cow::Borrowed("Iron Shovel"),
    description: Cow::Borrowed("An iron shovel. It's a bit of a mystery."),
    weight: 1,
};
pub const IRON_AXE: Item = Item {
    name: Cow::Borrowed("Iron Axe"),
    description: Cow::Borrowed("An iron axe. It's a bit of a mystery."),
    weight: 1,
};
pub const IRON_HELMET: Item = Item {
    name: Cow::Borrowed("Iron Helmet"),
    description: Cow::Borrowed("An iron helmet. It's a bit of a mystery."),
    weight: 5,
};
pub const IRON_ARMOR_PLATE: Item = Item {
    name: Cow::Borrowed("Iron Armor Plate"),
    description: Cow::Borrowed("An iron armor plate. It's a bit of a mystery."),
    weight: 50,
};
pub const IRON_SHIELD: Item = Item {
    name: Cow::Borrowed("Iron Shield"),
    description: Cow::Borrowed("An iron shield. It's a bit of a mystery."),
    weight: 20,
};
pub const IRON_BAR: Item = Item {
    name: Cow::Borrowed("Iron Bar"),
    description: Cow::Borrowed("An iron bar. It's a bit of a mystery."),
    weight: 1,
};
pub const IRON_ORE: Item = Item {
    name: Cow::Borrowed("Iron Ore"),
    description: Cow::Borrowed("An iron ore. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_INGOT: Item = Item {
    name: Cow::Borrowed("Gold Ingot"),
    description: Cow::Borrowed("A gold ingot. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_SWORD: Item = Item {
    name: Cow::Borrowed("Gold Sword"),
    description: Cow::Borrowed("A gold sword. It's a bit of a mystery."),
    weight: 5,
};
pub const GOLD_SHOVEL: Item = Item {
    name: Cow::Borrowed("Gold Shovel"),
    description: Cow::Borrowed("A gold shovel. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_PICK: Item = Item {
    name: Cow::Borrowed("Gold Pick"),
    description: Cow::Borrowed("A gold pick. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_AXE: Item = Item {
    name: Cow::Borrowed("Gold Axe"),
    description: Cow::Borrowed("A gold axe. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_HELMET: Item = Item {
    name: Cow::Borrowed("Gold Helmet"),
    description: Cow::Borrowed("A gold helmet. It's a bit of a mystery."),
    weight: 5,
};
pub const GOLD_ARMOR_PLATE: Item = Item {
    name: Cow::Borrowed("Gold Armor Plate"),
    description: Cow::Borrowed("A gold armor plate. It's a bit of a mystery."),
    weight: 50,
};
pub const GOLD_SHIELD: Item = Item {
    name: Cow::Borrowed("Gold Shield"),
    description: Cow::Borrowed("A gold shield. It's a bit of a mystery."),
    weight: 20,
};
pub const GOLD_BAR: Item = Item {
    name: Cow::Borrowed("Gold Bar"),
    description: Cow::Borrowed("A gold bar. It's a bit of a mystery."),
    weight: 1,
};
pub const GOLD_ORE: Item = Item {
    name: Cow::Borrowed("Gold Ore"),
    description: Cow::Borrowed("A gold ore. It's a bit of a mystery."),
    weight: 1,
};
pub const DIAMOND_NUGGET: Item = Item {
    name: Cow::Borrowed("Diamond Nugget"),
    description: Cow::Borrowed("A diamond nugget. It's a bit of a mystery."),
    weight: 1,
};
pub const DIAMOND_NUGGET_SMALL: Item = Item {
    name: Cow::Borrowed("Diamond Nugget Small"),
    description: Cow::Borrowed("A small diamond nugget. A holy item."),
    weight: 1,
};
pub const DIAMOND_NUGGET_MEDIUM: Item = Item {
    name: Cow::Borrowed("Diamond Nugget Medium"),
    description: Cow::Borrowed("A medium diamond nugget. A holy item."),
    weight: 1,
};
pub const DIAMOND_NUGGET_LARGE: Item = Item {
    name: Cow::Borrowed("Diamond Nugget Large"),
    description: Cow::Borrowed("A large diamond nugget. A holy item."),
    weight: 1,
};
pub const DIAMOND_SWORD: Item = Item {
    name: Cow::Borrowed("Diamond Sword"),
    description: Cow::Borrowed("A diamond sword. A holy item."),
    weight: 5,
};
pub const DIAMOND_PICK: Item = Item {
    name: Cow::Borrowed("Diamond Pick"),
    description: Cow::Borrowed("A diamond pick. A holy item."),
    weight: 1,
};
pub const DIAMOND_ORE: Item = Item {
    name: Cow::Borrowed("Diamond Ore"),
    description: Cow::Borrowed("A diamond ore. A holy item."),
    weight: 1,
};
pub const DIAMOND_SHIELD: Item = Item {
    name: Cow::Borrowed("Diamond Shield"),
    description: Cow::Borrowed("A diamond shield. A holy item."),
    weight: 20,
};
pub const STEEL_INGOT: Item = Item {
    name: Cow::Borrowed("Steel Ingot"),
    description: Cow::Borrowed("A steel ingot. It's a bit of a mystery."),
    weight: 1,
};
pub const STEEL_SWORD: Item = Item {
    name: Cow::Borrowed("Steel Sword"),
    description: Cow::Borrowed("A steel sword. It's a bit of a mystery."),
    weight: 5,
};
pub const STEEL_SHOVEL: Item = Item {
    name: Cow::Borrowed("Steel Shovel"),
    description: Cow::Borrowed("A steel shovel. It's a bit of a mystery."),
    weight: 1,
};
pub const STEEL_PICK: Item = Item {
    name: Cow::Borrowed("Steel Pick"),
    description: Cow::Borrowed("A steel pick. It's a bit of a mystery."),
    weight: 1,
};
pub const STEEL_AXE: Item = Item {
    name: Cow::Borrowed("Steel Axe"),
    description: Cow::Borrowed("A steel axe. It's a bit of a mystery."),
    weight: 1,
};
pub const STEEL_HELMET: Item = Item {
    name: Cow::Borrowed("Steel Helmet"),
    description: Cow::Borrowed("A steel helmet. It's a bit of a mystery."),
    weight: 5,
};
pub const STEEL_ARMOR_PLATE: Item = Item {
    name: Cow::Borrowed("Steel Armor Plate"),
    description: Cow::Borrowed("A steel armor plate. It's a bit of a mystery."),
    weight: 50,
};
pub const STEEL_SHIELD: Item = Item {
    name: Cow::Borrowed("Steel Shield"),
    description: Cow::Borrowed("A steel shield. It's a bit of a mystery."),
    weight: 20,
};
// food constants
pub const APPLE: Item = Item {
    name: Cow::Borrowed("Apple"),
    description: Cow::Borrowed("A delicious apple. It's a bit of a mystery."),
    weight: 1,
};
pub const BREAD: Item = Item {
    name: Cow::Borrowed("Bread"),
    description: Cow::Borrowed("A loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
pub const MEAT: Item = Item {
    name: Cow::Borrowed("Meat"),
    description: Cow::Borrowed("A piece of meat. It's a bit of a mystery."),
    weight: 1,
};
pub const POTATO: Item = Item {
    name: Cow::Borrowed("Potato"),
    description: Cow::Borrowed("A potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CARROT: Item = Item {
    name: Cow::Borrowed("Carrot"),
    description: Cow::Borrowed("A carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_MEAT: Item = Item {
    name: Cow::Borrowed("Cooked Meat"),
    description: Cow::Borrowed("A cooked piece of meat. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Potato"),
    description: Cow::Borrowed("A cooked potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Carrot"),
    description: Cow::Borrowed("A cooked carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BREAD: Item = Item {
    name: Cow::Borrowed("Cooked Bread"),
    description: Cow::Borrowed("A cooked loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_APPLE: Item = Item {
    name: Cow::Borrowed("Cooked Apple"),
    description: Cow::Borrowed("A cooked apple. Tastes best when combined with sugar."),
    weight: 1,
};
pub const COOKED_MEAT_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Meat Potato"),
    description: Cow::Borrowed("A cooked meat potato. It's a bit of a mystery."),
    weight: 1,
};
pub const BEANS: Item = Item {
    name: Cow::Borrowed("Beans"),
    description: Cow::Borrowed("A bunch of beans. It's a beant of a beenstery."),
    weight: 1,
};
pub const COOKED_BEANS: Item = Item {
    name: Cow::Borrowed("Cooked Beans"),
    description: Cow::Borrowed("A cooked bunch of beans. It's a bit of a mystery."),
    weight: 1,
};
pub const CHICKEN: Item = Item {
    name: Cow::Borrowed("Chicken"),
    description: Cow::Borrowed("A chicken. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHICKEN: Item = Item {
    name: Cow::Borrowed("Cooked Chicken"),
    description: Cow::Borrowed("A cooked chicken. It's a bit of a mystery."),
    weight: 1,
};
pub const PORK: Item = Item {
    name: Cow::Borrowed("Pork"),
    description: Cow::Borrowed("A piece of pork. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_PORK: Item = Item {
    name: Cow::Borrowed("Cooked Pork"),
    description: Cow::Borrowed("A cooked piece of pork. It's a bit of a mystery."),
    weight: 1,
};
pub const BEEF: Item = Item {
    name: Cow::Borrowed("Beef"),
    description: Cow::Borrowed("A piece of beef. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BEEF: Item = Item {
    name: Cow::Borrowed("Cooked Beef"),
    description: Cow::Borrowed("A cooked slab of beef. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE: Item = Item {
    name: Cow::Borrowed("Cheese"),
    description: Cow::Borrowed("A piece of cheese. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHICKEN_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Chicken Potato"),
    description: Cow::Borrowed("A cooked chicken potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CHICKEN_POTATO: Item = Item {
    name: Cow::Borrowed("Chicken Potato"),
    description: Cow::Borrowed("A chicken potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CHICKEN_BREAD: Item = Item {
    name: Cow::Borrowed("Chicken Bread"),
    description: Cow::Borrowed("A chicken loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHICKEN_BREAD: Item = Item {
    name: Cow::Borrowed("Cooked Chicken Bread"),
    description: Cow::Borrowed("A cooked chicken loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
// FIXME this will be a monster
pub const CHICKEN_APPLE: Item = Item {
    name: Cow::Borrowed("Chicken Apple"),
    description: Cow::Borrowed("A chicken apple. It's a bit of a mystery."),
    weight: 1,
};
pub const CHICKEN_MEAT_POTATO: Item = Item {
    name: Cow::Borrowed("Chicken Meat Potato"),
    description: Cow::Borrowed("A potato with chicken in it. Its quite tasty"),
    weight: 1,
};
pub const PEPPER: Item = Item {
    name: Cow::Borrowed("Pepper"),
    description: Cow::Borrowed("A pepper. It's a bit of a mystery."),
    weight: 1,
};
pub const SALT: Item = Item {
    name: Cow::Borrowed("Salt"),
    description: Cow::Borrowed("A salt. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Carrot"),
    description: Cow::Borrowed("A cooked carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_MEAT_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Meat Carrot"),
    description: Cow::Borrowed("A cooked meat carrot. Toxic for those not from the marshlands."),
    weight: 1,
};
pub const MUSHROOM: Item = Item {
    name: Cow::Borrowed("Mushroom"),
    description: Cow::Borrowed("A mushroom. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_MUSHROOM: Item = Item {
    name: Cow::Borrowed("Cooked Mushroom"),
    description: Cow::Borrowed("A cooked mushroom. It's a bit of a mystery."),
    weight: 1,
};
pub const BEER: Item = Item {
    name: Cow::Borrowed("Beer"),
    description: Cow::Borrowed("A bottle of beer. It's a bit of a mystery."),
    weight: 1,
};
pub const WINE: Item = Item {
    name: Cow::Borrowed("Wine"),
    description: Cow::Borrowed("A bottle of wine. It's a bit of a mystery."),
    weight: 1,
};
pub const COFFEE: Item = Item {
    name: Cow::Borrowed("Coffee"),
    description: Cow::Borrowed("A cup of coffee. It's a bit of a mystery."),
    weight: 1,
};
pub const CANDY: Item = Item {
    name: Cow::Borrowed("Candy"),
    description: Cow::Borrowed("A piece of candy. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_FISH: Item = Item {
    name: Cow::Borrowed("Cooked Fish"),
    description: Cow::Borrowed("A cooked fish. It's a bit of a mystery."),
    weight: 1,
};
pub const FISH: Item = Item {
    name: Cow::Borrowed("Fish"),
    description: Cow::Borrowed("A fish. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE: Item = Item {
    name: Cow::Borrowed("Chocolate"),
    description: Cow::Borrowed("A piece of chocolate. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE_CAKE: Item = Item {
    name: Cow::Borrowed("Chocolate Cake"),
    description: Cow::Borrowed("A chocolate cake. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE_CAKE_SLICE: Item = Item {
    name: Cow::Borrowed("Chocolate Cake Slice"),
    description: Cow::Borrowed("A chocolate cake slice. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_PORK_CHEESE: Item = Item {
    name: Cow::Borrowed("Cooked Pork Cheese"),
    description: Cow::Borrowed("A cooked pork cheese. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_PORK_CHICKEN: Item = Item {
    name: Cow::Borrowed("Cooked Pork Chicken"),
    description: Cow::Borrowed("A cooked pork chicken. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHICKEN_CHEESE: Item = Item {
    name: Cow::Borrowed("Cooked Chicken Cheese"),
    description: Cow::Borrowed("A cooked chicken cheese. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BEEF_CHEESE: Item = Item {
    name: Cow::Borrowed("Cooked Beef Cheese"),
    description: Cow::Borrowed("A cooked beef cheese. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BEEF_CHICKEN: Item = Item {
    name: Cow::Borrowed("Cooked Beef Chicken"),
    description: Cow::Borrowed("A cooked beef chicken. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BEEF_CHICKEN_CHEESE: Item = Item {
    name: Cow::Borrowed("Cooked Beef Chicken Cheese"),
    description: Cow::Borrowed("A cooked beef chicken cheese. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Potato"),
    description: Cow::Borrowed("A cooked cheese potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Carrot"),
    description: Cow::Borrowed("A cooked cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_PIE: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Pie"),
    description: Cow::Borrowed("A cooked cheese pie. It's a bit of a mystery."),
    weight: 1,
};
pub const POTATO_CHIPS: Item = Item {
    name: Cow::Borrowed("Potato Chips"),
    description: Cow::Borrowed("A bag of potato chips. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_POTATO_CHIPS: Item = Item {
    name: Cow::Borrowed("Cooked Potato Chips"),
    description: Cow::Borrowed("A bag of cooked potato chips. It's a bit of a mystery."),
    weight: 1,
};
pub const RICE: Item = Item {
    name: Cow::Borrowed("Rice"),
    description: Cow::Borrowed("A bag of rice. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_RICE: Item = Item {
    name: Cow::Borrowed("Cooked Rice"),
    description: Cow::Borrowed("A bag of cooked rice. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHICKEN_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Chicken Potato"),
    description: Cow::Borrowed("A cooked chicken potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_PORK_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Pork Potato"),
    description: Cow::Borrowed("A cooked pork potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_BEEF_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Beef Potato"),
    description: Cow::Borrowed("A cooked beef potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CHICKEN_MEAT: Item = Item {
    name: Cow::Borrowed("Chicken Meat"),
    description: Cow::Borrowed("A piece of chicken meat. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_POTATO: Item = Item {
    name: Cow::Borrowed("Cheese Potato"),
    description: Cow::Borrowed("A cheese potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Potato"),
    description: Cow::Borrowed("A cooked cheese potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cheese Carrot"),
    description: Cow::Borrowed("A cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Carrot"),
    description: Cow::Borrowed("A cooked cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_BREAD: Item = Item {
    name: Cow::Borrowed("Cheese Bread"),
    description: Cow::Borrowed("A delicious form of bread"),
    weight: 1,
};
pub const CHEESE_POTATO: Item = Item {
    name: Cow::Borrowed("Cheese Potato"),
    description: Cow::Borrowed("A cheese potato. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Potato"),
    description: Cow::Borrowed("A cooked cheese potato. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cheese Carrot"),
    description: Cow::Borrowed("A cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Carrot"),
    description: Cow::Borrowed("A cooked cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_BREAD: Item = Item {
    name: Cow::Borrowed("Cheese Bread"),
    description: Cow::Borrowed("A cheese loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_BREAD: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Bread"),
    description: Cow::Borrowed("A cooked cheese loaf of bread. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_APPLE: Item = Item {
    name: Cow::Borrowed("Cheese Apple"),
    description: Cow::Borrowed("A cheese apple. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHEESE_APPLE: Item = Item {
    name: Cow::Borrowed("Cooked Cheese Apple"),
    description: Cow::Borrowed("A cooked cheese apple. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_MEAT: Item = Item {
    name: Cow::Borrowed("Cheese Meat"),
    description: Cow::Borrowed("A strange item."),
    weight: 1,
};
pub const FRIED_EGGS: Item = Item {
    name: Cow::Borrowed("Fried Eggs"),
    description: Cow::Borrowed("A bag of fried eggs. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE: Item = Item {
    name: Cow::Borrowed("Chocolate"),
    description: Cow::Borrowed("A bag of chocolate. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CHOCOLATE: Item = Item {
    name: Cow::Borrowed("Cooked Chocolate"),
    description: Cow::Borrowed("A bag of chocolate. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE_CAKE: Item = Item {
    name: Cow::Borrowed("Chocolate Cake"),
    description: Cow::Borrowed("A chocolate cake. It's a bit of a mystery."),
    weight: 1,
};
pub const CHOCOLATE_CAKE_COOKIE: Item = Item {
    name: Cow::Borrowed("Chocolate Cake Cookie"),
    description: Cow::Borrowed("A chocolate cake cookie. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_COOKIE: Item = Item {
    name: Cow::Borrowed("Cooked Cookie"),
    description: Cow::Borrowed("A bag of cooked cookie. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKIE: Item = Item {
    name: Cow::Borrowed("Cookie"),
    description: Cow::Borrowed("A bag of cookie. It's a bit of a mystery."),
    weight: 1,
};
pub const CARROT_CHIPS: Item = Item {
    name: Cow::Borrowed("Carrot Chips"),
    description: Cow::Borrowed("A bag of carrot chips. It's a bit of a mystery."),
    weight: 1,
};
pub const COOKED_CARROT_CHIPS: Item = Item {
    name: Cow::Borrowed("Cooked Carrot Chips"),
    description: Cow::Borrowed("A bag of cooked carrot chips. It's a bit of a mystery."),
    weight: 1,
};
// potions
pub const POTION: Item = Item {
    name: Cow::Borrowed("Potion"),
    description: Cow::Borrowed("A potion. It's a bit of a mystery."),
    weight: 1,
};
pub const HEALING_POTION: Item = Item {
    name: Cow::Borrowed("Healing Potion"),
    description: Cow::Borrowed("A healing potion. It's a bit of a mystery."),
    weight: 1,
};
pub const HEALING_POTION_II: Item = Item {
    name: Cow::Borrowed("Healing Potion II"),
    description: Cow::Borrowed("A healing potion. It's a bit of a mystery."),
    weight: 1,
};
pub const HEALING_POTION_III: Item = Item {
    name: Cow::Borrowed("Healing Potion III"),
    description: Cow::Borrowed("A healing potion. It's a bit of a mystery."),
    weight: 1,
};
pub const HEALING_POTION_IV: Item = Item {
    name: Cow::Borrowed("Healing Potion IV"),
    description: Cow::Borrowed("A healing potion. It's a bit of a mystery."),
    weight: 1,
};
pub const HEALING_POTION_V: Item = Item {
    name: Cow::Borrowed("Healing Potion V"),
    description: Cow::Borrowed("A healing potion. It's a bit of a mystery."),
    weight: 1,
};
pub const MANA_POTION: Item = Item {
    name: Cow::Borrowed("Mana Potion"),
    description: Cow::Borrowed("A mana potion. It's a bit of a mystery."),
    weight: 1,
};
pub const MANA_POTION_II: Item = Item {
    name: Cow::Borrowed("Mana Potion II"),
    description: Cow::Borrowed("A mana potion. It's a bit of a mystery."),
    weight: 1,
};
pub const MANA_POTION_III: Item = Item {
    name: Cow::Borrowed("Mana Potion III"),
    description: Cow::Borrowed("A mana potion. It's a bit of a mystery."),
    weight: 1,
};
pub const MANA_POTION_IV: Item = Item {
    name: Cow::Borrowed("Mana Potion IV"),
    description: Cow::Borrowed("A mana potion. It's a bit of a mystery."),
    weight: 1,
};
pub const MANA_POTION_V: Item = Item {
    name: Cow::Borrowed("Mana Potion V"),
    description: Cow::Borrowed("A mana potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STAMINA_POTION: Item = Item {
    name: Cow::Borrowed("Stamina Potion"),
    description: Cow::Borrowed("A stamina potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STAMINA_POTION_II: Item = Item {
    name: Cow::Borrowed("Stamina Potion II"),
    description: Cow::Borrowed("A stamina potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STAMINA_POTION_III: Item = Item {
    name: Cow::Borrowed("Stamina Potion III"),
    description: Cow::Borrowed("A stamina potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STAMINA_POTION_IV: Item = Item {
    name: Cow::Borrowed("Stamina Potion IV"),
    description: Cow::Borrowed("A stamina potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STAMINA_POTION_V: Item = Item {
    name: Cow::Borrowed("Stamina Potion V"),
    description: Cow::Borrowed("A stamina potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STRENGTH: Item = Item {
    name: Cow::Borrowed("Strength Potion"),
    description: Cow::Borrowed("A strength potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STRENGTH_II: Item = Item {
    name: Cow::Borrowed("Strength Potion II"),
    description: Cow::Borrowed("A strength potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STRENGTH_III: Item = Item {
    name: Cow::Borrowed("Strength Potion III"),
    description: Cow::Borrowed("A strength potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STRENGTH_IV: Item = Item {
    name: Cow::Borrowed("Strength Potion IV"),
    description: Cow::Borrowed("A strength potion. It's a bit of a mystery."),
    weight: 1,
};
pub const STRENGTH_V: Item = Item {
    name: Cow::Borrowed("Strength Potion V"),
    description: Cow::Borrowed("A strength potion. It's a bit of a mystery."),
    weight: 1,
};
// rings
pub const RING_OF_HEALTH: Item = Item {
    name: Cow::Borrowed("Ring of Health"),
    description: Cow::Borrowed("A ring of health. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_MANA: Item = Item {
    name: Cow::Borrowed("Ring of Mana"),
    description: Cow::Borrowed("A ring of mana. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_STAMINA: Item = Item {
    name: Cow::Borrowed("Ring of Stamina"),
    description: Cow::Borrowed("A ring of stamina. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_STRENGTH: Item = Item {
    name: Cow::Borrowed("Ring of Strength"),
    description: Cow::Borrowed("A ring of strength. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_INVISIBILITY: Item = Item {
    name: Cow::Borrowed("Ring of Invisibility"),
    description: Cow::Borrowed("A ring of invisibility. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_MAGIC: Item = Item {
    name: Cow::Borrowed("Ring of Magic"),
    description: Cow::Borrowed("A ring of magic. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_SPEED: Item = Item {
    name: Cow::Borrowed("Ring of Speed"),
    description: Cow::Borrowed("A ring of speed. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_DEXTERITY: Item = Item {
    name: Cow::Borrowed("Ring of Dexterity"),
    description: Cow::Borrowed("A ring of dexterity. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_LUCK: Item = Item {
    name: Cow::Borrowed("Ring of Luck"),
    description: Cow::Borrowed("A ring of luck. It's a bit of a mystery."),
    weight: 1,
};
pub const RING_OF_LIFE: Item = Item {
    name: Cow::Borrowed("Ring of Life"),
    description: Cow::Borrowed("A ring of life. It's a bit of a mystery."),
    weight: 1,
};

// Impl

impl GameState for Game {
    fn init(_engine: &mut Engine) -> Self where Self: Sized {
        Self { }
    }
}

// functions

fn ThreeCoreBasicBrewingTable() {}
fn ThreeCoreMediumBrewingTable() {}
fn ThreeCoreAdvancedBrewingTable() {}
fn ThreeCoreGodlyBrewingTable() {}
fn FiveCoreBasicBrewingTable() {}
fn FiveCoreMediumBrewingTable() {}
fn FiveCoreAdvancedBrewingTable() {}
fn FiveCoreGodlyBrewingTable() {}
fn TenCoreBasicBrewingTable() {}
fn TenCoreMediumBrewingTable() {}
fn TenCoreAdvancedBrewingTable() {}
fn TenCoreGodlyBrewingTable() {}
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
fn anvil_recipe_get(id: u32) -> RecipeStruct {}
fn cooking_recipe_get(id: u32) -> RecipeStruct {}
fn potion_recipe_get(id: u32) -> RecipeStruct {}
fn smelting_recipe_get(id: u32) -> RecipeStruct {}
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
fn main() {
    Framework::<Game>::new()
        .unwrap()
        .title("Simple")
        .run();
}


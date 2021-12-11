use std::borrow::Cow;
pub struct Item {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    weight: u32,
}
// Constants
// weapon and armor constants
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
pub const DIAMOND_SHIELD: Item = Item {
    name: Cow::Borrowed("Diamond Shield"),
    description: Cow::Borrowed("A diamond shield. A holy item."),
    weight: 20,
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
pub const CHEESE_CARROT: Item = Item {
    name: Cow::Borrowed("Cheese Carrot"),
    description: Cow::Borrowed("A cheese carrot. It's a bit of a mystery."),
    weight: 1,
};
pub const CHEESE_BREAD: Item = Item {
    name: Cow::Borrowed("Cheese Bread"),
    description: Cow::Borrowed("A delicious form of bread"),
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
pub const COOKED_CHOCOLATE: Item = Item {
    name: Cow::Borrowed("Cooked Chocolate"),
    description: Cow::Borrowed("A bag of chocolate. It's a bit of a mystery."),
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
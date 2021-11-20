pub struct Item {
    name: Cow<'static, str>,
    description: Cow<'static, str>,
    weight: u32,
}
// weapon and armor constants
pub const RUSTY_SWORD: Item = Item {
    name: "Rusty Sword",
    description: "A rusty sword.",
    weight: 5,
};
pub const FOWL_EGG: Item = Item {
    name: "Fowl Egg",
    description: "A fowl egg.",
    weight: 1,
};
pub const FRIED_FOWL_EGGS: Item = Item {
    name: "Fried Fowl Egg",
    description: "A fried fowl egg. Good for a quick snack and tastes best with salt.",
    weight: 1,
}
pub const RUSTY_KEY: Item = Item {
    name: "Rusty Key",
    description: "A rusty key. Unsure what it unlocks.",
    weight: 1,
};
pub const RUSTY_PICK: Item = Item {
    name: "Rusty Pick",
    description: "A rusty pick. Will break inside any lock you put it in.",
    weight: 1,
};
pub const RUSTY_SHOVEL: Item = Item {
    name: "Rusty Shovel",
    description: "A rusty shovel. It's strangely durable",
    weight: 1,
};
pub const RUSTY_AXE: Item = Item {
    name: "Rusty Axe",
    description: "A rusty axe. It's strangely durable",
    weight: 1,
};
pub const SPECIAL_RUSTY_KEY: Item = Item {
    name: "Special Rusty Key",
    description: "A special rusty key. It's a key that unlocks something special.",
    weight: 1,
};
pub const DRAGON_SCALE: Item = Item {
    name: "Dragon Scale",
    description: "A dragon scale. It's a bit of a mystery.",
    weight: 3,
};
pub const DRAGON_SCALE_ARMOR: Item = Item {
    name: "Dragon Scale Armor",
    description: "A dragon scale armor. Forged from the scales of the strongest beast, it's a reminder of our accomplishments.",
    weight: 50,
};
pub const DRAGON_SCALE_SHIELD: Item = Item {
    name: "Dragon Scale Shield",
    description: "A dragon scale shield. Forged from the scales of the strongest beast, will block any fire attack up to rank 30 and any magic attack up to rank 5.",
    weight: 20,
};
pub const DRAGON_SCALE_HELMET: Item = Item {
    name: "Dragon Scale Helmet",
    description: "A dragon scale helmet. Forged from the scales of the strongest beast, it's a reminder of our accomplishments.",
    weight: 5,
};
pub const DRAGON_SCALE_ARMOR_PLATE: Item = Item {
    name: "Dragon Scale Armor Plate",
    description: "A dragon scale armor plate. Forged from the scales of the strongest beast, it's a reminder of our accomplishments.",
    weight: 50,
};
pub const DRAGON_BONE: Item = Item {
    name: "Dragon bone",
    description: "The bones of the legendary beasts of the skies.",
    weight: 5,
};
pub const DRAGON_BONE_ARMOR: Item = Item {
    name: "Dragon bone armor",
    description: "The bones of the dragon being used as a defense item brings out great strength in even the weakest of warriors",
    weight: 50,
};
pub const IRON_INGOT: Item = Item {
    name: "Iron Ingot",
    description: "An iron ingot. It's a bit of a mystery.",
    weight: 1,
};
pub const IRON_SWORD: Item = Item {
    name: "Iron Sword",
    description: "An iron sword. It's a bit of a mystery.",
    weight: 5,
};
pub const IRON_SHOVEL: Item = Item {
    name: "Iron Shovel",
    description: "An iron shovel. It's a bit of a mystery.",
    weight: 1,
};
pub const IRON_AXE: Item = Item {
    name: "Iron Axe",
    description: "An iron axe. It's a bit of a mystery.",
    weight: 1,
};
pub const IRON_HELMET: Item = Item {
    name: "Iron Helmet",
    description: "An iron helmet. It's a bit of a mystery.",
    weight: 5,
};
pub const IRON_ARMOR_PLATE: Item = Item {
    name: "Iron Armor Plate",
    description: "An iron armor plate. It's a bit of a mystery.",
    weight: 50,
};
pub const IRON_SHIELD: Item = Item {
    name: "Iron Shield",
    description: "An iron shield. It's a bit of a mystery.",
    weight: 20,
};
pub const IRON_BAR: Item = Item {
    name: "Iron Bar",
    description: "An iron bar. It's a bit of a mystery.",
    weight: 1,
};
pub const IRON_ORE: Item = Item {
    name: "Iron Ore",
    description: "An iron ore. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_INGOT: Item = Item {
    name: "Gold Ingot",
    description: "A gold ingot. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_SWORD: Item = Item {
    name: "Gold Sword",
    description: "A gold sword. It's a bit of a mystery.",
    weight: 5,
};
pub const GOLD_SHOVEL: Item = Item {
    name: "Gold Shovel",
    description: "A gold shovel. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_PICK: Item = Item {
    name: "Gold Pick",
    description: "A gold pick. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_AXE: Item = Item {
    name: "Gold Axe",
    description: "A gold axe. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_HELMET: Item = Item {
    name: "Gold Helmet",
    description: "A gold helmet. It's a bit of a mystery.",
    weight: 5,
};
pub const GOLD_ARMOR_PLATE: Item = Item {
    name: "Gold Armor Plate",
    description: "A gold armor plate. It's a bit of a mystery.",
    weight: 50,
};
pub const GOLD_SHIELD: Item = Item {
    name: "Gold Shield",
    description: "A gold shield. It's a bit of a mystery.",
    weight: 20,
};
pub const GOLD_BAR: Item = Item {
    name: "Gold Bar",
    description: "A gold bar. It's a bit of a mystery.",
    weight: 1,
};
pub const GOLD_ORE: Item = Item {
    name: "Gold Ore",
    description: "A gold ore. It's a bit of a mystery.",
    weight: 1,
};
pub const DIAMOND_NUGGET: Item = Item {
    name: "Diamond Nugget",
    description: "A diamond nugget. It's a bit of a mystery.",
    weight: 1,
};
pub const DIAMOND_NUGGET_SMALL: Item = Item {
    name: "Diamond Nugget Small",
    description: "A small diamond nugget. A holy item.",
    weight: 1,
};
pub const DIAMOND_NUGGET_MEDIUM: Item = Item {
    name: "Diamond Nugget Medium",
    description: "A medium diamond nugget. A holy item.",
    weight: 1,
};
pub const DIAMOND_NUGGET_LARGE: Item = Item {
    name: "Diamond Nugget Large",
    description: "A large diamond nugget. A holy item.",
    weight: 1,
};
pub const DIAMOND_SWORD: Item = Item {
    name: "Diamond Sword",
    description: "A diamond sword. A holy item.",
    weight: 5,
};
pub const DIAMOND_PICK: Item = Item {
    name: "Diamond Pick",
    description: "A diamond pick. A holy item.",
    weight: 1,
};
pub const DIAMOND_ORE: Item = Item {
    name: "Diamond Ore",
    description: "A diamond ore. A holy item.",
    weight: 1,
};
pub const DIAMOND_SHIELD: Item = Item {
    name: "Diamond Shield",
    description: "A diamond shield. A holy item.",
    weight: 20,
};
pub const STEEL_INGOT: Item = Item {
    name: "Steel Ingot",
    description: "A steel ingot. It's a bit of a mystery.",
    weight: 1,
};
pub const STEEL_SWORD: Item = Item {
    name: "Steel Sword",
    description: "A steel sword. It's a bit of a mystery.",
    weight: 5,
};
pub const STEEL_SHOVEL: Item = Item {
    name: "Steel Shovel",
    description: "A steel shovel. It's a bit of a mystery.",
    weight: 1,
};
pub const STEEL_PICK: Item = Item {
    name: "Steel Pick",
    description: "A steel pick. It's a bit of a mystery.",
    weight: 1,
};
pub const STEEL_AXE: Item = Item {
    name: "Steel Axe",
    description: "A steel axe. It's a bit of a mystery.",
    weight: 1,
};
pub const STEEL_HELMET: Item = Item {
    name: "Steel Helmet",
    description: "A steel helmet. It's a bit of a mystery.",
    weight: 5,
};
pub const STEEL_ARMOR_PLATE: Item = Item {
    name: "Steel Armor Plate",
    description: "A steel armor plate. It's a bit of a mystery.",
    weight: 50,
};
pub const STEEL_SHIELD: Item = Item {
    name: "Steel Shield",
    description: "A steel shield. It's a bit of a mystery.",
    weight: 20,
};
// food constants
pub const APPLE: Item = Item {
    name: "Apple",
    description: "A delicious apple. It's a bit of a mystery.",
    weight: 1,
};
pub const BREAD: Item = Item {
    name: "Bread",
    description: "A loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
pub const MEAT: Item = Item {
    name: "Meat",
    description: "A piece of meat. It's a bit of a mystery.",
    weight: 1,
};
pub const POTATO: Item = Item {
    name: "Potato",
    description: "A potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CARROT: Item = Item {
    name: "Carrot",
    description: "A carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_MEAT: Item = Item {
    name: "Cooked Meat",
    description: "A cooked piece of meat. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_POTATO: Item = Item {
    name: "Cooked Potato",
    description: "A cooked potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CARROT: Item = Item {
    name: "Cooked Carrot",
    description: "A cooked carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BREAD: Item = Item {
    name: "Cooked Bread",
    description: "A cooked loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_APPLE: Item = Item {
    name: "Cooked Apple",
    description: "A cooked apple. Tastes best when combined with sugar.",
    weight: 1,
};
pub const COOKED_MEAT_POTATO: Item = Item {
    name: "Cooked Meat Potato",
    description: "A cooked meat potato. It's a bit of a mystery.",
    weight: 1,
};
pub const BEANS: Item = Item {
    name: "Beans",
    description: "A bunch of beans. It's a beant of a beenstery.",
    weight: 1,
};
pub const COOKED_BEANS: Item = Item {
    name: "Cooked Beans",
    description: "A cooked bunch of beans. It's a bit of a mystery.",
    weight: 1,
};
pub const CHICKEN: Item = Item {
    name: "Chicken",
    description: "A chicken. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHICKEN: Item = Item {
    name: "Cooked Chicken",
    description: "A cooked chicken. It's a bit of a mystery.",
    weight: 1,
};
pub const PORK: Item = Item {
    name: "Pork",
    description: "A piece of pork. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_PORK: Item = Item {
    name: "Cooked Pork",
    description: "A cooked piece of pork. It's a bit of a mystery.",
    weight: 1,
};
pub const BEEF: Item = Item {
    name: "Beef",
    description: "A piece of beef. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BEEF: Item = Item {
    name: "Cooked Beef",
    description: "A cooked slab of beef. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE: Item = Item {
    name: "Cheese",
    description: "A piece of cheese. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHICKEN_POTATO: Item = Item {
    name: "Cooked Chicken Potato",
    description: "A cooked chicken potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CHICKEN_POTATO: Item = Item {
    name: "Chicken Potato",
    description: "A chicken potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CHICKEN_BREAD: Item = Item {
    name: "Chicken Bread",
    description: "A chicken loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHICKEN_BREAD: Item = Item {
    name: "Cooked Chicken Bread",
    description: "A cooked chicken loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
// FIXME this will be a monster
pub const CHICKEN_APPLE: Item = Item {
    name: "Chicken Apple",
    description: "A chicken apple. It's a bit of a mystery.",
    weight: 1,
};
pub const CHICKEN_MEAT_POTATO: Item = Item {
    name: "Chicken Meat Potato",
    description: "A potato with chicken in it. Its quite tasty",
    weight: 1,
};
pub const PEPPER: Item = Item {
    name: "Pepper",
    description: "A pepper. It's a bit of a mystery.",
    weight: 1,
};
pub const SALT: Item = Item {
    name: "Salt",
    description: "A salt. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CARROT: Item = Item {
    name: "Cooked Carrot",
    description: "A cooked carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_MEAT_CARROT: Item = Item {
    name: "Cooked Meat Carrot",
    description: "A cooked meat carrot. Toxic for those not from the marshlands.",
    weight: 1,
};
pub const MUSHROOM: Item = Item {
    name: "Mushroom",
    description: "A mushroom. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_MUSHROOM: Item = Item {
    name: "Cooked Mushroom",
    description: "A cooked mushroom. It's a bit of a mystery.",
    weight: 1,
};
pub const BEER: Item = Item {
    name: "Beer",
    description: "A bottle of beer. It's a bit of a mystery.",
    weight: 1,
};
pub const WINE: Item = Item {
    name: "Wine",
    description: "A bottle of wine. It's a bit of a mystery.",
    weight: 1,
};
pub const COFFEE: Item = Item {
    name: "Coffee",
    description: "A cup of coffee. It's a bit of a mystery.",
    weight: 1,
};
pub const CANDY: Item = Item {
    name: "Candy",
    description: "A piece of candy. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_FISH: Item = Item {
    name: "Cooked Fish",
    description: "A cooked fish. It's a bit of a mystery.",
    weight: 1,
};
pub const FISH: Item = Item {
    name: "Fish",
    description: "A fish. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE: Item = Item {
    name: "Chocolate",
    description: "A piece of chocolate. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE_CAKE: Item = Item {
    name: "Chocolate Cake",
    description: "A chocolate cake. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE_CAKE_SLICE: Item = Item {
    name: "Chocolate Cake Slice",
    description: "A chocolate cake slice. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_PORK_CHEESE: Item = Item {
    name: "Cooked Pork Cheese",
    description: "A cooked pork cheese. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_PORK_CHICKEN: Item = Item {
    name: "Cooked Pork Chicken",
    description: "A cooked pork chicken. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHICKEN_CHEESE: Item = Item {
    name: "Cooked Chicken Cheese",
    description: "A cooked chicken cheese. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BEEF_CHEESE: Item = Item {
    name: "Cooked Beef Cheese",
    description: "A cooked beef cheese. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BEEF_CHICKEN: Item = Item {
    name: "Cooked Beef Chicken",
    description: "A cooked beef chicken. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BEEF_CHICKEN_CHEESE: Item = Item {
    name: "Cooked Beef Chicken Cheese",
    description: "A cooked beef chicken cheese. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: "Cooked Cheese Potato",
    description: "A cooked cheese potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: "Cooked Cheese Carrot",
    description: "A cooked cheese carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_PIE: Item = Item {
    name: "Cooked Cheese Pie",
    description: "A cooked cheese pie. It's a bit of a mystery.",
    weight: 1,
};
pub const POTATO_CHIPS: Item = Item {
    name: "Potato Chips",
    description: "A bag of potato chips. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_POTATO_CHIPS: Item = Item {
    name: "Cooked Potato Chips",
    description: "A bag of cooked potato chips. It's a bit of a mystery.",
    weight: 1,
};
pub const RICE: Item = Item {
    name: "Rice",
    description: "A bag of rice. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_RICE: Item = Item {
    name: "Cooked Rice",
    description: "A bag of cooked rice. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHICKEN_POTATO: Item = Item {
    name: "Cooked Chicken Potato",
    description: "A cooked chicken potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_PORK_POTATO: Item = Item {
    name: "Cooked Pork Potato",
    description: "A cooked pork potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_BEEF_POTATO: Item = Item {
    name: "Cooked Beef Potato",
    description: "A cooked beef potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CHICKEN_MEAT: Item = Item {
    name: "Chicken Meat",
    description: "A piece of chicken meat. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_POTATO: Item = Item {
    name: "Cheese Potato",
    description: "A cheese potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: "Cooked Cheese Potato",
    description: "A cooked cheese potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_CARROT: Item = Item {
    name: "Cheese Carrot",
    description: "A cheese carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: "Cooked Cheese Carrot",
    description: "A cooked cheese carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_BREAD: Item = Item {
    name: "Cheese Bread",
    description: "A delicious form of bread",
    weight: 1,
};
pub const CHEESE_POTATO: Item = Item {
    name: "Cheese Potato",
    description: "A cheese potato. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_POTATO: Item = Item {
    name: "Cooked Cheese Potato",
    description: "A cooked cheese potato. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_CARROT: Item = Item {
    name: "Cheese Carrot",
    description: "A cheese carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_CARROT: Item = Item {
    name: "Cooked Cheese Carrot",
    description: "A cooked cheese carrot. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_BREAD: Item = Item {
    name: "Cheese Bread",
    description: "A cheese loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_BREAD: Item = Item {
    name: "Cooked Cheese Bread",
    description: "A cooked cheese loaf of bread. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_APPLE: Item = Item {
    name: "Cheese Apple",
    description: "A cheese apple. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHEESE_APPLE: Item = Item {
    name: "Cooked Cheese Apple",
    description: "A cooked cheese apple. It's a bit of a mystery.",
    weight: 1,
};
pub const CHEESE_MEAT: Item = Item {
    name: "Cheese Meat",
    description: "A strange item."
    weight: 1,
};
pub const FRIED_EGGS: Item = Item {
    name: "Fried Eggs",
    description: "A bag of fried eggs. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE: Item = Item {
    name: "Chocolate",
    description: "A bag of chocolate. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CHOCOLATE: Item = Item {
    name: "Cooked Chocolate",
    description: "A bag of chocolate. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE_CAKE: Item = Item {
    name: "Chocolate Cake",
    description: "A chocolate cake. It's a bit of a mystery.",
    weight: 1,
};
pub const CHOCOLATE_CAKE_COOKIE: Item = Item {
    name: "Chocolate Cake Cookie",
    description: "A chocolate cake cookie. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_COOKIE: Item = Item {
    name: "Cooked Cookie",
    description: "A bag of cooked cookie. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKIE: Item = Item {
    name: "Cookie",
    description: "A bag of cookie. It's a bit of a mystery.",
    weight: 1,
};
pub const CARROT_CHIPS: Item = Item {
    name: "Carrot Chips",
    description: "A bag of carrot chips. It's a bit of a mystery.",
    weight: 1,
};
pub const COOKED_CARROT_CHIPS: Item = Item {
    name: "Cooked Carrot Chips",
    description: "A bag of cooked carrot chips. It's a bit of a mystery.",
    weight: 1,
};
// potions
pub const POTION: Item = Item {
    name: "Potion",
    description: "A potion. It's a bit of a mystery.",
    weight: 1,
};
pub const HEALING_POTION: Item = Item {
    name: "Healing Potion",
    description: "A healing potion. It's a bit of a mystery.",
    weight: 1,
};
pub const HEALING_POTION_II: Item = Item {
    name: "Healing Potion II",
    description: "A healing potion. It's a bit of a mystery.",
    weight: 1,
};
pub const HEALING_POTION_III: Item = Item {
    name: "Healing Potion III",
    description: "A healing potion. It's a bit of a mystery.",
    weight: 1,
};
pub const HEALING_POTION_IV: Item = Item {
    name: "Healing Potion IV",
    description: "A healing potion. It's a bit of a mystery.",
    weight: 1,
};
pub const HEALING_POTION_V: Item = Item {
    name: "Healing Potion V",
    description: "A healing potion. It's a bit of a mystery.",
    weight: 1,
};
pub const MANA_POTION: Item = Item {
    name: "Mana Potion",
    description: "A mana potion. It's a bit of a mystery.",
    weight: 1,
};
pub const MANA_POTION_II: Item = Item {
    name: "Mana Potion II",
    description: "A mana potion. It's a bit of a mystery.",
    weight: 1,
};
pub const MANA_POTION_III: Item = Item {
    name: "Mana Potion III",
    description: "A mana potion. It's a bit of a mystery.",
    weight: 1,
};
pub const MANA_POTION_IV: Item = Item {
    name: "Mana Potion IV",
    description: "A mana potion. It's a bit of a mystery.",
    weight: 1,
};
pub const MANA_POTION_V: Item = Item {
    name: "Mana Potion V",
    description: "A mana potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STAMINA_POTION: Item = Item {
    name: "Stamina Potion",
    description: "A stamina potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STAMINA_POTION_II: Item = Item {
    name: "Stamina Potion II",
    description: "A stamina potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STAMINA_POTION_III: Item = Item {
    name: "Stamina Potion III",
    description: "A stamina potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STAMINA_POTION_IV: Item = Item {
    name: "Stamina Potion IV",
    description: "A stamina potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STAMINA_POTION_V: Item = Item {
    name: "Stamina Potion V",
    description: "A stamina potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STRENGTH: Item = Item {
    name: "Strength Potion",
    description: "A strength potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STRENGTH_II: Item = Item {
    name: "Strength Potion II",
    description: "A strength potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STRENGTH_III: Item = Item {
    name: "Strength Potion III",
    description: "A strength potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STRENGTH_IV: Item = Item {
    name: "Strength Potion IV",
    description: "A strength potion. It's a bit of a mystery.",
    weight: 1,
};
pub const STRENGTH_V: Item = Item {
    name: "Strength Potion V",
    description: "A strength potion. It's a bit of a mystery.",
    weight: 1,
};
// rings
pub const RING_OF_HEALTH: Item = Item {
    name: "Ring of Health",
    description: "A ring of health. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_MANA: Item = Item {
    name: "Ring of Mana",
    description: "A ring of mana. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_STAMINA: Item = Item {
    name: "Ring of Stamina",
    description: "A ring of stamina. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_STRENGTH: Item = Item {
    name: "Ring of Strength",
    description: "A ring of strength. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_INVISIBILITY: Item = Item {
    name: "Ring of Invisibility",
    description: "A ring of invisibility. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_MAGIC: Item = Item {
    name: "Ring of Magic",
    description: "A ring of magic. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_SPEED: Item = Item {
    name: "Ring of Speed",
    description: "A ring of speed. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_DEXTERITY: Item = Item {
    name: "Ring of Dexterity",
    description: "A ring of dexterity. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_LUCK: Item = Item {
    name: "Ring of Luck",
    description: "A ring of luck. It's a bit of a mystery.",
    weight: 1,
};
pub const RING_OF_LIFE: Item = Item {
    name: "Ring of Life",
    description: "A ring of life. It's a bit of a mystery.",
    weight: 1,
};
use crate::items::weapon::definition::WeaponKind;
use fyrox::core::visitor::Visitor;
use fyrox::core::visitor::prelude::Visit;
use fyrox::core::visitor::VisitResult;
use crate::items::apparel::definition::ApparelKind;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Visit)]
pub enum ItemKind {
	RustyKey,
	RustyPick,
	SpecialRustyKey,
	DragonScale,
	DragonBone,
	IronIngot,
	IronOre,
	IronBar,
	GoldIngot,
	GoldOre,
	DiamondNugget ,
	DiamondNuggetSmall,
	DiamondNuggetLarge,
	DiamondOre,
	SteelIngot,
	SteelBar,
	SteelOre,
	RustySword,
	Rustyshovel,
	Rustyaxe,
	Ironsword,
	Ironshovel,
	Ironaxe,
	Goldsword,
	Goldshovel,
	Goldaxe,
	Diamondsword,
	Obsidianknife,
	Steelaxe,
	Steelshovel,
	Steelsword,
	Mithrilsword,
	Mithrilaxe,
	Obsidiansword,
	Obsidianaxe,
	Adamantiumsword,
	Adamantiumaxe,
	Ironhelmet,
	Ironboots,
	Irongreaves,
	Irongauntlets,
	Ironshield,
	Ironchestplate,
	Ironleggings,
	DragonScalehelmet,
	DragonScaleBoots,
	DragonScaleGreaves,
	DragonScaleGauntlets,
	DragonScaleShield,
	DragonScaleChestplate,
	DragonScaleLeggings,
}
impl Default for ItemKind {
	fn default() -> Self {
		Self::RustyKey
	}
}
impl ItemKind {
	pub fn associated_weapon(&self) -> Option<WeaponKind> {
		match self {
			ItemKind::RustySword => Some(WeaponKind::RustySword),
			ItemKind::Rustyshovel => Some(WeaponKind::Rustyshovel),
			ItemKind::Rustyaxe => Some(WeaponKind::Rustyaxe),
			ItemKind::Ironsword => Some(WeaponKind::Ironsword),
			ItemKind::Ironshovel => Some(WeaponKind::Ironshovel),
			ItemKind::Ironaxe => Some(WeaponKind::Ironaxe),
			ItemKind::Goldsword => Some(WeaponKind::Goldsword),
			ItemKind::Goldshovel => Some(WeaponKind::Goldshovel),
			ItemKind::Goldaxe => Some(WeaponKind::Goldaxe),
			ItemKind::Diamondsword => Some(WeaponKind::Diamondsword),
			ItemKind::Obsidianknife => Some(WeaponKind::Obsidianknife),
			ItemKind::Steelaxe => Some(WeaponKind::Steelaxe),
			ItemKind::Steelshovel => Some(WeaponKind::Steelshovel),
			ItemKind::Steelsword => Some(WeaponKind::Steelsword),
			ItemKind::Mithrilsword => Some(WeaponKind::Mithrilsword),
			ItemKind::Mithrilaxe => Some(WeaponKind::Mithrilaxe),
			ItemKind::Obsidiansword => Some(WeaponKind::Obsidiansword),
			ItemKind::Obsidianaxe => Some(WeaponKind::Obsidianaxe),
			ItemKind::Adamantiumsword => Some(WeaponKind::Adamantiumsword),
			ItemKind::Adamantiumaxe => Some(WeaponKind::Adamantiumaxe),
			ItemKind::Ironhelmet
			| ItemKind::Ironboots
			| ItemKind::Irongreaves
			| ItemKind::Irongauntlets
			| ItemKind::Ironshield
			| ItemKind::Ironchestplate
			| ItemKind::Ironleggings
			| ItemKind::DragonScalehelmet
			| ItemKind::DragonScaleBoots
			| ItemKind::DragonScaleGreaves
			| ItemKind::DragonScaleGauntlets
			| ItemKind::DragonScaleShield
			| ItemKind::DragonScaleChestplate
			| ItemKind::DragonScaleLeggings
			| ItemKind::RustyKey
			| ItemKind::RustyPick
			| ItemKind::SpecialRustyKey
			| ItemKind::DragonScale
			| ItemKind::DragonBone
			| ItemKind::IronIngot
			| ItemKind::IronOre
			| ItemKind::IronBar
			| ItemKind::GoldIngot
			| ItemKind::GoldOre
			| ItemKind::DiamondNugget
			| ItemKind::DiamondNuggetSmall
			| ItemKind::DiamondNuggetLarge
			| ItemKind::DiamondOre
			| ItemKind::SteelIngot
			| ItemKind::SteelBar
			| ItemKind::SteelOre => None,
		}
	}
	pub fn associated_apparel(&self) -> Option<ApparelKind> {
		match self {
			ItemKind::Ironhelmet => Some(ApparelKind::Ironhelmet),
			ItemKind::Ironboots => Some(ApparelKind::Ironboots),
			ItemKind::Irongreaves => Some(ApparelKind::Irongreaves),
			ItemKind::Irongauntlets => Some(ApparelKind::Irongauntlets),
			ItemKind::Ironshield => Some(ApparelKind::Ironshield),
			ItemKind::Ironchestplate => Some(ApparelKind::Ironchestplate),
			ItemKind::Ironleggings => Some(ApparelKind::Ironleggings),
			ItemKind::DragonScalehelmet => Some(ApparelKind::DragonScalehelmet),
			ItemKind::DragonScaleBoots => Some(ApparelKind::DragonScaleBoots),
			ItemKind::DragonScaleGreaves => Some(ApparelKind::DragonScaleGreaves),
			ItemKind::DragonScaleGauntlets => Some(ApparelKind::DragonScaleGauntlets),
			ItemKind::DragonScaleShield => Some(ApparelKind::DragonScaleShield),
			ItemKind::DragonScaleChestplate => Some(ApparelKind::DragonScaleChestplate),
			ItemKind::DragonScaleLeggings => Some(ApparelKind::DragonScaleLeggings),
			ItemKind::RustyKey
			| ItemKind::RustyPick
			| ItemKind::SpecialRustyKey
			| ItemKind::DragonScale
			| ItemKind::DragonBone
			| ItemKind::IronIngot
			| ItemKind::IronOre
			| ItemKind::IronBar
			| ItemKind::GoldIngot
			| ItemKind::GoldOre
			| ItemKind::DiamondNugget 
			| ItemKind::DiamondNuggetSmall
			| ItemKind::DiamondNuggetLarge
			| ItemKind::DiamondOre
			| ItemKind::SteelIngot
			| ItemKind::SteelBar
			| ItemKind::SteelOre
			| ItemKind::RustySword
			| ItemKind::Rustyshovel
			| ItemKind::Rustyaxe
			| ItemKind::Ironsword
			| ItemKind::Ironshovel
			| ItemKind::Ironaxe
			| ItemKind::Goldsword
			| ItemKind::Goldshovel
			| ItemKind::Goldaxe
			| ItemKind::Diamondsword
			| ItemKind::Obsidianknife
			| ItemKind::Steelaxe
			| ItemKind::Steelshovel
			| ItemKind::Steelsword
			| ItemKind::Mithrilsword
			| ItemKind::Mithrilaxe
			| ItemKind::Obsidiansword
			| ItemKind::Obsidianaxe
			| ItemKind::Adamantiumsword
			| ItemKind::Adamantiumaxe => None,
		}
	}
}

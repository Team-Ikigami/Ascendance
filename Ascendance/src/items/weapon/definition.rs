use crate::items::definition::ItemKind;
use fyrox::core::visitor::prelude::{
	Visit,
	Visitor,
	VisitResult,
};
use serde::Deserialize;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Visit)]
#[repr(u32)]
pub enum WeaponKind {
	RustySword = 0,
	Rustyshovel = 1,
	Rustyaxe = 2,
	Ironsword = 3,
	Ironshovel = 4,
	Ironaxe = 5,
	Goldsword = 6,
	Goldshovel = 7,
	Goldaxe = 8,
	Diamondsword = 9,
	Obsidianknife = 10,
	Steelaxe = 11,
	Steelshovel = 12,
	Steelsword = 13,
	Mithrilsword = 14,
	Mithrilaxe = 15,
	Obsidiansword = 16,
	Obsidianaxe = 17,
	Adamantiumsword = 18,
	Adamantiumaxe = 19,
}

impl Default for WeaponKind {
	fn default() -> Self {
		WeaponKind::RustySword
	}
}
impl WeaponKind {
	pub fn associated_item(&self) -> ItemKind {
		match self {
			WeaponKind::RustySword => ItemKind::RustySword,
			WeaponKind::Rustyshovel => ItemKind::Rustyshovel,
			WeaponKind::Rustyaxe => ItemKind::Rustyaxe,
			WeaponKind::Ironsword => ItemKind::Ironsword,
			WeaponKind::Ironshovel => ItemKind::Ironshovel,
			WeaponKind::Ironaxe => ItemKind::Ironaxe,
			WeaponKind::Goldsword => ItemKind::Goldsword,
			WeaponKind::Goldshovel => ItemKind::Goldshovel,
			WeaponKind::Goldaxe => ItemKind::Goldaxe,
			WeaponKind::Diamondsword => ItemKind::Diamondsword,
			WeaponKind::Obsidianknife => ItemKind::Obsidianknife,
			WeaponKind::Steelaxe => ItemKind::Steelaxe,
			WeaponKind::Steelshovel => ItemKind::Steelshovel,
			WeaponKind::Steelsword => ItemKind::Steelsword,
			WeaponKind::Mithrilsword => ItemKind::Mithrilsword,
			WeaponKind::Mithrilaxe => ItemKind::Mithrilaxe,
			WeaponKind::Obsidiansword => ItemKind::Obsidiansword,
			WeaponKind::Obsidianaxe => ItemKind::Obsidianaxe,
			WeaponKind::Adamantiumsword => ItemKind::Adamantiumsword,
			WeaponKind::Adamantiumaxe => ItemKind::Adamantiumaxe,
		}
	}
}
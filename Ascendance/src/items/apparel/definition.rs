use crate::items::definition::ItemKind;
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Visit)]
#[repr(u32)]
pub enum ApparelKind {
	Ironhelmet = 0,
	Ironboots = 1,
	Irongreaves = 2,
	Irongauntlets = 3,
	Ironshield = 4,
	Ironchestplate = 5,
	Ironleggings = 6,
	DragonScalehelmet = 7,
	DragonScaleBoots = 8,
	DragonScaleGreaves = 9,
	DragonScaleGauntlets = 10,
	DragonScaleShield = 11,
	DragonScaleChestplate = 12,
	DragonScaleLeggings = 13,
}
impl Default for ApparelKind {
	fn default() -> Self {
		ApparelKind::Ironhelmet
	}
}
impl ApparelKind {
	pub fn associated_item(&self) -> ItemKind {
		match self {
			ApparelKind::Ironhelmet => ItemKind::IronHelmet,
			ApparelKind::Ironboots => ItemKind::IronBoots,
			ApparelKind::Irongreaves => ItemKind::IronGreaves,
			ApparelKind::Irongauntlets => ItemKind::IronGauntlets,
			ApparelKind::Ironshield => ItemKind::IronShield,
			ApparelKind::Ironchestplate => ItemKind::IronChestplate,
			ApparelKind::Ironleggings => ItemKind::IronLeggings,
			ApparelKind::DragonScalehelmet => ItemKind::DragonScaleHelmet,
			ApparelKind::DragonScaleBoots => ItemKind::DragonScaleBoots,
			ApparelKind::DragonScaleGreaves => ItemKind::DragonScaleGreaves,
			ApparelKind::DragonScaleGauntlets => ItemKind::DragonScaleGauntlets,
			ApparelKind::DragonScaleShield => ItemKind::DragonScaleShield,
			ApparelKind::DragonScaleChestplate => ItemKind::DragonScaleChestplate,
			ApparelKind::DragonScaleLeggings => ItemKind::DragonScaleLeggings,
		}
	}
}
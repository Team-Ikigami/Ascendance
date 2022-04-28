use std::fs::File;
use inv_sys::{
	Inv,
	ItemStack,
	Slot,
	InvAccessErr,
	InvOverflow,
	StackErr,
	Stacksize,
};
use crate::items::ItemEnum;
use std::path::PathBuf;

impl Stacksize for ItemEnum {
	fn get_max_stacksize(&self) -> usize {
		100
	}
}

#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Visit)]
pub struct ItemEntry {
	pub kind: ItemEnum,
	pub amount: u32,
}
pub struct Inventory {
	items: Vec<ItemEntry>,
}
pub struct SaveInventory { 
	pub inventory_path: PathBuf,
}

impl ItemEntry {
	pub fn kind(&self) -> ItemEnum {
		self.kind
	}
	pub fn amount(&self) -> u32 {
		self.amount
	}
}
impl Inventory {
	pub fn new() -> Self {
		Self { items: Vec![ ] }
	}
	pub fn from_inner(items: Vec<ItemEntry>) -> Self {
		Self { items}
	}	
}

impl SaveInventory {
	pub fn CreateInventory(&self) -> Self where: Sized {
		let mut inv = Inv::<ItemEnum>::new(32);
		Self {
			inventory_path: Default::default(),
		}
	}
	pub fn Save(&self) {
		let printPath = self.inventory_path;
		println!("{}", printPath)
	}
}
impl Default for SaveInventory {
	fn default() -> Self {
		Self {
			inventory_path: std::path::Path("bins/saves/chr1.sav")
		}
	}
}

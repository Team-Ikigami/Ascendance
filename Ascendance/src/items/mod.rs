pub mod weapon;
use weapon::definition::WeaponKind;
use fyrox::scene::node::Node;
pub mod definition;
pub mod apparel;
use definition::ItemKind;
use fyrox::core::visitor::Visit;
use fyrox::core::{
	color::Color,
	pool::{Handle, PoolIterator, PoolIteratorMut},
	algebra::Vector3,
	visitor::{VisitResult, Visitor},
};
use lazy_static::lazy_static;
use std::fs::File;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Visit)]
pub struct Item {
	kind: ItemKind,
	pivot: Handle<Node>,
	model: Handle<Node>,
	spark: Handle<Node>,
	spark_size_change_dir: f32,
	pub stack_size: u32,
	#[visit(skip)]
	pub definition: &'static ItemDefinition,
}
impl Default for Item {
	fn default() -> Self {
		Self {
			kind: ItemKind::Rustykey,
			pivot: Default::default(),
			model: Default::default(),
			spark: Default::default(),
			spark_size_change_dir: 1.0,
			stack_size: 1,
			definition: Self::get_definition(ItemKind::Rustykey),
		}
	}
}
#[derive(Deserialize)]
pub struct ItemDefinition {
	pub model: String,
	pub descripttion: String,
	pub scale: f32,
	pub name: String,
	pub consumble: bool,
	pub preview: String,
}
#[derive(Deserialize, Default)]
pub struct ItemDefinitionContainer {
	map: HashMap<ItemKind, ItemDefinition>,
}
impl ItemDefinitionContainer {
	pub fn new() -> Self {
		let file = File::open("data/configs/items.ron").unwrap();
		ron::de::from_reader(file).unwrap()
	}
}
lazy_static! {
	static ref DEFINITIONS: ItemDefinitionContainer = ItemDefinitionContainer::new();
}

impl Item {
	pub fn get_definition(kind: ItemKind) -> &'static ItemDefinition {
		DEFINITIONS
			.map
			.get(&kind)
			.expect(&format!("No definition for item kind {:?}", kind))
	}
}

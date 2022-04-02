use crate::weapon::AttackTypes;

#[derive(Clone)]
struct HardNecromancer {
	health: u32,
	magic: u32,
	strength: u32,
	resistances: AttackTypes,
	speed: u32,
	weaknesses: AttackTypes,
	gender: bool,
	name: String,
	necromancer_type: u32,
}
impl HardNecromancer {
	fn new() -> Self {
		Self {
			health: 0,
			magic: 0,
			strength: 0,
			resistances: AttackTypes{
				fire: true,
				water: false,
				cold: false,
				lightning: true,
				poison: false,
				psychic: false,
				acid: false,
				necrotic: false,
				bash: true,
				piercing: true,
				slashing: false,
			},
			speed: 0,
			weaknesses: AttackTypes{
				fire: false,
				water: false,
				cold: false,
				lightning: false,
				poison: false,
				psychic: false,
				acid: false,
				necrotic: false,
				bash: false,
				piercing: false,
				slashing: false,
			},
			gender: true,
			name: String::from(""),
			necromancer_type: 0,
		}
	}
	pub fn set_health(&mut self, health: u32) -> Self {
		self.health = health;
		self
	}
	pub fn set_speed(&mut self, speed: u32) -> Self {
        self.speed = speed;
        self
    }
	pub fn set_weaknesses(&mut self, weaknesses: AttackTypes) -> Self {
		self.weaknesses = weaknesses;
		self
	}
	pub fn set_gender(&mut self, gender: bool) -> Self {
		self.gender = gender;
		self
	}
	pub fn set_necromancer_type(&mut self, necromancer_type: u32) -> Self {
		self.necromancer_type = necromancer_type;
		self
	}
	pub fn set_name(&mut self, name: String) -> Self {
		let mut name = String::from(" Hard ");
		if name == " Hard " {
			
			match self.necromancer_type {
				1 => { name.push_str("Ghost Whisperer ") },
				2 => { name.push_str("Conjurer ") },
				3 => { name.push_str("Necromancer ") },
				4 => { name.push_str("Raiser of the Dead ") },
				5 => { name.push_str("Reaper of Souls ") },
			};
			match self.health {
				self.health =< 200 => { name = String::from("Weak ") + name.as_str() },
				self.health =< 400 => { name = String::from("Injured ") + name.as_str() },
				self.health =< 600 => { name = String::from("Healthy ") + name.as_str() },
				self.health =< 800 => { name = String::from("Youthful ") + name.as_str() },
				self.health =< 1000 => { name = String::from("Rejuvenated ") + name.as_str() },
			};
			match self.magic {
				self.magic =< 200 -> name.push_str(" Exhausted "),
				self.magic =< 400 -> name.push_str(" Tired "),
				self.magic =< 600 -> name.push_str(" Ready "),
				self.magic =< 800 -> name.push_str(" Enchanted "),
				self.magic =< 1000 -> name.push_str(" Invigorated "),
			};
		}
		
		self.name = name;
		self
	}
}
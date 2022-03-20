use crate::weapon::AttackTypes;
use fyrox::engine::Engine;
use rand::thread_rng;
use rand::Rng;
struct GodOfTheDeep {
    name: String,
    description: String,
    level: u32,
    hit_points: u32,
    speed: u32,
    strength: u32,
    proficiencies: AttackTypes,
    damage_vulnerabilities: AttackTypes,
    damage_resistances: AttackTypes,
    damage_immunities: AttackTypes,
    senses: AttackTypes,
}
impl GodOfTheDeep {
    fn build() -> Self
    where
        Self: Sized,
    {
        let mut rng = rand::thread_rng();
        let mut health = rng.gen_range(500..20000);
        let mut magic = rng.gen_range(1500..2000);
        let resists = AttackTypes {
            fire: true,
            water: true,
            cold: true,
            lightning: false,
            poison: false,
            psychic: true,
            acid: false,
            necrotic: false,
            bash: true,
            piercing: false,
            slashing: true,
        };
        let weakness = AttackTypes {
            fire: false,
            water: false,
            cold: false,
            lightning: true,
            poison: false,
            psychic: false,
            acid: true,
            necrotic: true,
            bash: false,
            piercing: true,
            slashing: false,
        };
        let immune = AttackTypes {
            fire: true,
            water: true,
            cold: true,
            lightning: false,
            poison: false,
            psychic: false,
            acid: false,
            necrotic: false,
            bash: false,
            piercing: false,
            slashing: false,
        };
        let strengths = AttackTypes {
            fire: false,
            water: true,
            cold: true,
            lightning: false,
            poison: false,
            psychic: true,
            acid: true,
            necrotic: false,
            bash: true,
            piercing: false,
            slashing: false,
        };
        Self {
			name: String::from("God of the Deep"),
			description: String::from("The all powerful being that resides at the depths of the ocean where no light may appear. It is often described as a hideous beast that can eat hundreds of ships in a single mouthful and has been said to have been as vast as the great blue itself"),
			level: 1,
			hit_points: health,
			speed: 30,
			strength: 30,
			proficiencies: strengths,
			damage_vulnerabilities: weakness,
			damage_resistances: resists,
			damage_immunities: immune,
			senses: resists,
		}
    }
    fn bubble_attack_one(engine: &mut Engine) {
        engine.get_window().set_title("Bubble Attack One");
    }
}

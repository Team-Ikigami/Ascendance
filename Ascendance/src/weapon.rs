// attack types. Used by enemies and player.
#[derive(Debug, Copy, Clone)]
pub struct AttackTypes {
	pub fire: bool,
	pub water: bool,
	pub cold: bool,
	pub lightning: bool,
	pub poison: bool,
	pub psychic: bool,
	pub acid: bool,
	pub necrotic: bool,
	pub bash: bool,
	pub piercing: bool,
	pub slashing: bool,
}
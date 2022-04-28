#[derive(Clone)]
pub enum SaveKind {
    Complete,
    Character,
    Inventory,
    World
}
#[derive(Clone)]
pub struct SaveState {
    bad_exit: bool,
    save_kind: SaveKind
}
impl Default for SaveState {
    fn default(&self) -> Self {
        Self {
            bad_exit: false,
            save_kind: SaveKind::Complete,
        }
    }
}
impl SaveState {
    pub fn save(&self) -> Self {
        Self {
            bad_exit: Default::default(),
            save_kind: Default::default(),
        }
    }
    pub fn set_save_kind(&self) -> Self {
        match self.save_kind {
            SaveKind::complete => println!("Game Saved"),
            SaveKind::character => println!("Character saved"),
            SaveKind::inventory => println!("Inventory Updated"),
            SaveKind::world => println!("World state saved"),
            _ => println!("nothing should happen and this should never occur"),
        }
    }
}

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
    save_kind: SaveKind,
    file: String,
    written: String,
}
impl Default for SaveKind {
	fn default(&self) -> Self {
		
	}
}
impl Default for SaveState {
    fn default(&self) -> Self {
        Self {
            bad_exit: false,
            save_kind: SaveKind::Complete,
            file: "data/saves/save_1.bin" 
        }
    }
}
impl SaveState {
    /// The file contains IN THIS ORDER the following:
    /// - player name (16 Bytes)
    /// - Inventory (16384 Bytes)
    ///     - 32 Bytes per item
    ///     - 24 Bytes for name
    ///     - 8 Bytes for quantity (starting with 0's)
    /// - NPC (3 NPC so 384 Bytes
    ///     - 128 Bytes per NPC
    ///     - 2 bytes for level of hatred
    ///     - 8 bytes for ID
    ///     - 2 bytes for number of respawns left
    ///     - 2 bytes for hostile or not
    ///     - 114 Bytes for inventory locks
    /// - bad_exit or not (1 byte)
    pub fn save(&self) -> Self {
        let mut inventory = InventoryWriter::get_written_inventory();
        let mut npc_state = crate::entitygen::npc::get_all_npc_states();
        let player_name = crate::player::Player::get_name();
        let file = OpenOption::new().write(true)
                                    .create(true)
                                    .open("data/saves/{}/{}_save.bin", player_name, player_name);
        let mut written = 0;
        while written < 17235 {
            file.write_all(b"0");
        }
        file.write_all(&player_name[0..16]);
        file.write_all(&inventory[17..16851]);
        file.write_all(&npc_state[16852..17236]);
        file.append(b"0");
        Self {
            bad_exit: Default::default(),
            save_kind: Default::default(),
        }
    }
    pub fn set_save_kind(&self) -> Self {
        match self.save_kind {
            SaveKind::Complete => println!("Game Saved"),
            SaveKind::Character => println!("Character saved"),
            SaveKind::Inventory => println!("Inventory Updated"),
            SaveKind::World => println!("World state saved"),
            _ => println!("nothing should happen and this should never occur"),
        }
    }
}

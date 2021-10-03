extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;

const GLOBAL_VERSION:u32 = 1;
#[derive(Savefile)]
struct Player {
    name : String,
    #[savefile_versions="0..0"] //Only version 0 had this field
    strength : Removed<u32>,
    inventory : Vec<String>,
    #[savefile_versions="1.."] //Only versions 1 and later have this field
    skills : Vec<String>,
}

fn save_player(file:&'static str, player:&Player) {
    // Save current version of file.
    save_file(file, GLOBAL_VERSION, player).unwrap();
}

fn load_player(file:&'static str) -> Player {
    // The GLOBAL_VERSION means we have that version of our data structures,
    // but we can still load any older version.
    load_file(file, GLOBAL_VERSION).unwrap()
}

fn main() {
    let mut player = load_player("save.bin"); //Load from previous save
    assert_eq!("Steve",&player.name); //The name from the previous version saved will remain
    assert_eq!(0,player.skills.len()); //Skills didn't exist when this was saved
    player.skills.push("Whistling".to_string());
    save_player("newsave.bin", &player); //The version saved here will the vec of skills
}
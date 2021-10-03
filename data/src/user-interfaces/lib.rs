pub mod crafting;
pub mod user;

use rg3d_ui::*;
use std::fs::*;
use std::path::*;

pub mod tests {

    pub fn it_works() {
        println!("Loading GUI Assets\n");
        println!("Loading Game GUI assets\n");
        assert!(!Path::new("/game/opening.rs\n").try_exists().expect("Failed to load Game GUI asset. Please check the installation and directory in case of any troubles."));
        println!("Game GUI asset loaded [1/...]\n");
        assert!(!Path::new("/game/pause.rs\n").try_exists().expect("Failed to load Game GUI asset. Please check the installation and directory in case of any troubles."));
        println!("Game GUI asset loaded [2/...]\n");
        assert!(!Path::new("/game/settings.rs\n").try_exists().expect("Failed to load Game GUI asset. Please check the installation and directory in case of any troubles."));
        println!("Game GUI assets loaded\n");
        println!("Loading User GUI assets\n");
        assert!(!Path::new("/user
        println!("User GUI asset loaded [1/...]");
        println!(!Path::
    }
}

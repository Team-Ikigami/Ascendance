pub use crate::items;
pub use crate::render;
pub use crate::user_interfaces;
pub use crate::world_interaction;
use std::fs::*;

let gameopen = 0;

pub mod game {
    pub fn run() {
        let mut file = File::create("/int/init.txt");
        let mut f = File::read
        if Path::path("/int/init.txt").exists() == true {
            let mut int = read_to_string(path: "/int/init.txt")
            if int = 1 {
                self::secondrun::tmpnm();
            }
            else {
                file(())
                println!("oops, it seems you havent done something wrong. We will now terminate your processes");
            }
        }
        else {
            self::wininitial::tmpnm();
        }
    }
    pub mod wininitial {
        pub fn tmpnm() {
            println!("gggg");
        }
    }
    pub mod secondrun {
        pub fn tmpnm() {
            println!("gggg");
        }
    }
}
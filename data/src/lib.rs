pub use items::*;
pub use render::*;
pub use user_interfaces::*;
pub use world_interaction::*;
use std::fs::*;
use rg3d::*;

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
                file("/int/init.txt")
                println!("oops, it seems you havent done something wrong. We will now terminate your processes");
            }
        }
        else {
            self::wininitial::tmpnm();
        }
    }
    pub mod wininitial {
        pub fn tmpnm() {
            ;
        }
    }
    pub mod secondrun {
        pub fn tmpnm() {
            println!("gggg");
        }
        pub fn openanimation() {
            // display animation
            secondrun::Frontpage();
        }
        pub fn Frontpage(){
            items::game::openingf::allaspects::Init();
            
        }
    }
}
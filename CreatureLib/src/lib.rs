//! npc means humans who are not inhenrently hostile or a random encounter
//!
mod npc;
mod hostile;
mod friends;
mod ai;
mod error;
use npc::{
    town::{
        random::new,
        cityone::*,
        citytwo::*,
    },
    random::{
        traveller::{Salesman, thief, lostman, crazylady},
    },
};
use hostile::{
    bandit::{BasicB, LeaderB, Chief, king, god},
    enemysoldier::{BasicES, LeaderES, ChiefES),
    large::{
        savannha::{Lion, Goblin, Tiger, Deranged},
        ocean::{Kraken, Shark, ReallyLargeShark},
        mountain::{Mountaindragon, Snowgiant},
        everwhere::{Troll, SleepingLandDragon},
    },
    small::{
        Mouse,
        FlashyBird, 
        Snakes,
        CuteShark,
        CommonFish,
        SparklyRareFish,
        rat,
        vulture,
        
    },
};
use friends::{
    Critter::{
        Spider,
        Worms,
        Flies,
    },
    large::{
        Cow,
        Bull,
        Sheep,
    },
    small::{
        squirrel,
    },
};
    
fn mountain(summonid: &mut CreatureID, creaturesize: &mut CreatureSize, &mut Hostility) {
    let summonid;
    match EnemySummon {
        Hostility(H) if H = True => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => Rat::new(),
                summonid(SID) if SID = 2 => Vulture::new(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => MountainDragon::new(),
                summonid(SID) if SID = 2 => SnowGiant::new(),
            }
        }
        Hostility(H) if H = False => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => Spider::New(),
                summon(SID) if SID = 2 => Worms::New(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => Sheep::New(rand::RandInt(10 .. 20));
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}

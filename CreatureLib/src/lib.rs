//! npc means humans who are not inhenrently hostile or a random encounter
//!
mod npc;
mod hostile;
mod friends;
mod error;
use npc::{
    town::{
        random,
        cityone,
        citytwo,
    },
    random::traveller,
};
use hostile::{
    bandit::{BasicB, LeaderB, Chief, king, god},
    enemysoldier::{BasicES, LeaderES, ChiefES),
    large::{
        Lion,
        Goblin,
        Tiger,
        Deranged,
        Kraken,
        Shark,
        ReallyLargeShark,
        Mountaindragon,
        Snowgiant,
        Troll,
        SleepingLandDragon,
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
fn savannha(summonid: &mut CreatureID, creaturesize: &mut CreatureSize, &mut Hostility) {
    let summonid;
    match EnemySummon {
        Hostility(H) if H = True => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
        }
        Hostility(H) if H = False => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::New(),
                summon(SID) if SID = 2 => ::New(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::New(rand::RandInt(10 .. 20));
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}
fn everywhere(summonid: &mut CreatureID, creaturesize: &mut CreatureSize, &mut Hostility) {
    let summonid;
    match EnemySummon {
        Hostility(H) if H = True => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
        }
        Hostility(H) if H = False => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::New(),
                summon(SID) if SID = 2 => ::New(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::New(rand::RandInt(10 .. 20));
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}
fn ocean(summonid: &mut CreatureID, creaturesize: &mut CreatureSize, &mut Hostility) {
    let summonid;
    match EnemySummon {
        Hostility(H) if H = True => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
        }
        Hostility(H) if H = False => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::New(),
                summon(SID) if SID = 2 => ::New(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::New(rand::RandInt(10 .. 20));
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}
fn underground(summonid: &mut CreatureID, creaturesize: &mut CreatureSize, &mut Hostility) {
    let summonid;
    match EnemySummon {
        Hostility(H) if H = True => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::new(),
                summonid(SID) if SID = 2 => ::new(),
            }
        }
        Hostility(H) if H = False => match {
            CreatureSize(CS) if CS = 1 => match {
                summonid(SID) if SID = 1 => ::New(),
                summon(SID) if SID = 2 => ::New(),
            }
            CreatureSize(CS) if CS = 2 => match {
                summonid(SID) if SID = 1 => ::New(rand::RandInt(10 .. 20));
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}
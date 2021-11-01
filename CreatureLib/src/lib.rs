//! npc means humans who are not inhenrently hostile or a random encounter
//! CS = 1 means small CS = 2 means big
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
    large::{Lion, Goblin, Tiger, Deranged, Kraken, Shark, ReallyLargeShark, Mountaindragon, Snowgiant, Troll, SleepingLandDragon},
    small::{Mouse, FlashyBird, Snakes, CuteShark, CommonFish, SparklyRareFish, rat, vulture},
};
use friends::{
    Critter::{Spider, Worms, Flies},
    large::{Cow, Bull, Sheep},
    small::{squirrel},
};
    
fn mountain(&CreatureID, &CreatureSize, &Hostility) {
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
fn savannha(&CreatureID, &CreatureSize, &Hostility) {
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
fn everywhere(&CreatureID, &CreatureSize, &Hostility) {
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
fn ocean(&CreatureID, &CreatureSize, &Hostility) {
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
fn underground(&CreatureID, &CreatureSize, &Hostility) {
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
                summonid(SID) if SID = 1 => ::New();
            }
        }
        _ => {
            let control_loop = ControlFlow::Exit;
        }
    }
}
fn banditfort(PlayerLevel, BanditFort) {
    match BFORT {
        BanditFort(BF) if BF = 1 => {
            let mut bandit = 0
            let mut leader = 0
            while(bandit < 10) {
                BanditB::new()
                    .clothes(rand::thread_rng().gen_range(1..4))
                    .weapon(rand::thread_rng().gen_range(1..5))
                    .health(PlayerHealth / 2)
                    .defense(PlayerDefense / 2)
                    .experiencegain(HitsGiven / 4)
                    .build()
                let bandit = bandit + 1
            }
            while(leader < 1) {
                LeaderB::new()
                    .clothes(rand::thread_rng().gen_range(1..4))
                    .weapon(rand::thread_rng().gen_range(1..5))
                    .health(PlayerHealth / 2)
                    .defense(PlayerDefense / 2)
                    .experiencegain(HitsGiven / 4) 
                    .build()
                let bandit = bandit + 1;
            }
        }
        BanditFort(BF) if BF = 2 => {
            while(bandit < 30) {
                BanditB::new()
                    .clothes(rand::thread_rng().gen_range(1..4))
                    .weapon(rand::thread_rng().gen_range(1..5))
                    .health(PlayerHealth / 2)
                    .defense(PlayerDefense / 2)
                    .build()
            }
            while(leader < 1) {
                LeaderB::new()
                    .clothes(rand::thread_rng().gen_range(1..4))
                    .weapon(rand::thread_rng().gen_range(1..5))
                    .health(PlayerHealth / 1.5)
                    .defense(PlayerDefense / 2)
                    .experiencegain(HitsGiven / 4) 
                    .build()
                let leader = leader + 1;
            }
        }
    }
}

use super::super::backend::*;

pub mod weapons {
    pub struct Weapontype {
        weaponused: char,
        weaponstrength: u32,
        bow: String,
        longbow: String,
        sword: String,
        longsword: String,
        shortsword: String,
        dagger: String,
        hammer: String,
        warhammer: String,
        cutlery: String,
        mace: String,
        waraxe: String,
        battleaxe: String,
        axe: String,
        magic: bool,
    };
    pub struct Magic {
        element: char,
        firespell: String,
        waterspell: String,
        icespell: String,
        electricspell: String,
        darkspell: String,
        spellpower: u64,
    }
    pub fn weaponused(weaponused: Char) -> Weapontype {
        Weapontype {
            weaponused: super::user_interfaces::user::interface::rows::slots.selectedweapon,
        }
    }
}
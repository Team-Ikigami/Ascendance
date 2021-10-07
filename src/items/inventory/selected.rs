pub mod backend;

pub mod weapons {
    pub struct Weapontype {
        weaponused: Char,
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
        element: Char,
        firespell: String,
        waterspell: String,
        icespell: String,
        electricspell: String,
        darkspell: String,
        spellpower: u64,
    };
    pub fn weaponused(weaponused: Char) -> weapontype {
        weapontype {
            weaponused: Super::user_interfaces::user::interface::rows::slots.selectedweapon,
        }
    }
}
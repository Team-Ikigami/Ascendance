use crate::inventory;
use super::items;

pub struct equipment() {
    head: u32;
    body: u32;
    pants: u32;
    shoes: u32;
    hair: u32;
    neckaccessories: u32;
    hairaccessories: u32;
    belt: u32;
    weapontype: items::inventory::selected::weapontype();
    weapon: items::inventory::selected::weapon();
}
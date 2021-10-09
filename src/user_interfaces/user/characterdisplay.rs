use super::super::super::items::*;

pub struct Equipment {
    head: u32,
    body: u32,
    pants: u32,
    shoes: u32,
    hair: u32,
    neckaccessories: u32,
    hairaccessories: u32,
    belt: u32,
    weapontype: itemsinventory::selected::weapontype(),
    weapon: itemsinventory::selected::weapon(),
}
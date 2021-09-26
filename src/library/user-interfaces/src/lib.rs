#[warn(not(unused_imports))]
pub mod basicbrewingtable;
pub mod mediumbrewingtable;
pub mod advancedbrewingtable;
pub mod godlybrewingtable;
pub mod basicmetalanvil;
pub mod mediummetalanvil;
pub mod advancedmetalanvil;
pub mod godlymetalanvil;
pub mod inventory;

use rg3d_ui::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

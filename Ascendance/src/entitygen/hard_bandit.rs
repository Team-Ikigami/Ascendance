struct FemaleHardBanditBuilder;
struct MaleHardBanditBuilder;
impl FemaleHardBanditBuilder {
    fn new(self: &mut Self) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn armour(&self) {}
    fn weapon(&self) {}
    fn statistics(&self, health: i32, magic: i32) {
        let magic = magic;
        let health = health;
        println!("{}", health);
        println!("{}", magic);
    }
}
impl MaleHardBanditBuilder {
    fn new(self: &mut Self) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn armour(&self) {}
    fn weapon(&self) {}
    fn statistics(&self, health: i32, magic: i32) {
        let magic = magic + 1;
        let health = health + 1;
        println!("{}", health);
        println!("{}", magic);
    }
}

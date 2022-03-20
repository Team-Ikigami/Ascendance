struct FriendlyMaleBanditBuilder;
struct FriendlyFemaleBanditBuilder;
impl FriendlyMaleBanditBuilder {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn ai_type(&mut self) {}
    fn armour(&mut self) {}
    fn weapon(&mut self) {}
    fn stats(&mut self, health: &mut f32, magic: &mut f32, speed: &mut f32) {}
}
impl FriendlyFemaleBanditBuilder {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn ai_type(&mut self) {}
    fn armour(&mut self) {}
    fn weapon(&mut self) {}
    fn stats(&mut self, health: &mut f32, magic: &mut f32, speed: &mut f32) {}
}

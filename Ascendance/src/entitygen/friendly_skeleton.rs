struct FriendlySkeletonBuilder;
impl FriendlySkeletonBuilder {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn stats(&mut self, health: &mut f32, magic: &mut f32, speed: &mut f32) {}
}

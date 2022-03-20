struct BasicNecromancerBuilder;

impl BasicNecromancerBuilder {
    fn new() {}
    fn ai_type(&mut self, ai: u32) {
        match ai {
            1 => {
                println!("Ranged Attack");
            }
            2 => {
                println!("Close up");
            }
            3 => {
                println!("Mixed Attack");
            }
            _ => {}
        }
    }
    fn armour(&mut self) {}
    fn weapon(&mut self) {}
    fn stats(&mut self, health: &mut f32, magic: &mut f32, speed: &mut f32) {}
}

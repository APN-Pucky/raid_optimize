
#[derive(Debug)]
pub struct Statistics {
    damage_done: u32,
    damage_taken: u32,
    healing_done: u32,
    healing_taken: u32,
    shielding_done: u32,
    shielding_taken: u32,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            damage_done: 0,
            damage_taken: 0,
            healing_done: 0,
            healing_taken: 0,
            shielding_done: 0,
            shielding_taken: 0,
        }
    }
}
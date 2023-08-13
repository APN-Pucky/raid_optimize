
#[derive(Debug)]
pub struct Statistics {
    pub damage_done: u32,
    pub damage_taken: u32,
    pub healing_done: u32,
    pub healing_taken: u32,
    pub shielding_done: u32,
    pub shielding_taken: u32,
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
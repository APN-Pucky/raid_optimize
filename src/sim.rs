
use crate::hero::Hero;
use crate::wave::Wave;
use crate::wave::Result;
use crate::hero::statistics::Statistics;

pub struct Sim<'a> {
    allies: &'a Vec<&'a Hero>,
    enemies: &'a Vec<&'a Hero>,
    iterations: u32,
    results : Vec<Result>,
    statistics: Vec<SimStatistics>,
}

pub struct SimStatistics {
    pub iterations: u32,
    pub damage_done: u32,
    pub damage_done_sq: u32,
    pub damage_taken: u32,
    pub damage_taken_sq: u32,
    pub healing_done: u32,
    pub healing_done_sq: u32,
    pub healing_taken: u32,
    pub healing_taken_sq: u32,
    pub shielding_done: u32,
    pub shielding_done_sq: u32,
    pub shielding_taken: u32,
    pub shielding_taken_sq: u32,
}

pub fn get_mean(sum : u32, N: u32) -> f32 {
    (sum as f32) / N as f32
}

pub fn get_standard_deviation(sum : u32, sum_sq:u32, N: u32) -> f32 {
    ((sum_sq as f32 - sum as f32 * sum as f32 / N as f32) / N as f32).sqrt()
}

pub fn get_mean_and_standard_deviation(sum : u32, sum_sq:u32, N: u32) -> (f32, f32) {
    (get_mean(sum, N), get_standard_deviation(sum, sum_sq, N))
}

impl  SimStatistics {
pub fn get_damage_done(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.damage_done, self.damage_done_sq, self.iterations)
}
pub fn get_damage_taken(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.damage_taken, self.damage_taken_sq, self.iterations)
}
pub fn get_healing_done(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.healing_done, self.healing_done_sq, self.iterations)
}
pub fn get_healing_taken(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.healing_taken, self.healing_taken_sq, self.iterations)
}
pub fn get_shielding_done(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.shielding_done, self.shielding_done_sq, self.iterations)
}
pub fn get_shielding_taken(&self) -> (f32, f32) {
    get_mean_and_standard_deviation(self.shielding_taken, self.shielding_taken_sq, self.iterations)
}
pub fn print(&self) {
    let (mean, std) = self.get_damage_done();
    println!("\t damage done: {} +- {}", mean,std);
    let (mean, std) = self.get_damage_taken();
    println!("\t damage taken: {} +- {}", mean,std);
    let (mean, std) = self.get_healing_done();
    println!("\t healing done: {} +- {}", mean,std);
    let (mean, std) = self.get_healing_taken();
    println!("\t healing taken: {} +- {}", mean,std);
    let (mean, std) = self.get_shielding_done();
    println!("\t shielding done: {} +- {}", mean,std);
    let (mean, std) = self.get_shielding_taken();
    println!("\t shielding taken: {} +- {}", mean,std);
}
}


impl Sim<'_> {
    pub fn new<'a>(allies: &'a Vec<&'a Hero>, enemies : &'a Vec<&'a Hero> ) -> Sim<'a> {
        // create statistcs vector with one entry per hero
        let mut statistics = Vec::new();
        for _ in 0..(allies.len() + enemies.len()) {
            statistics.push(SimStatistics {
                iterations :0 ,
                damage_done: 0,
                damage_done_sq: 0,
                damage_taken: 0,
                damage_taken_sq: 0,
                healing_done: 0,
                healing_done_sq: 0,
                healing_taken: 0,
                healing_taken_sq: 0,
                shielding_done: 0,
                shielding_done_sq: 0,
                shielding_taken: 0,
                shielding_taken_sq: 0,
            });
        }
        Sim {
            allies: allies,
            enemies: enemies,
            iterations : 1000,
            results : Vec::new(),
            statistics : statistics,
        }
    }

    pub fn add_statistics( statistics : &mut Vec<SimStatistics>,  added : &Vec<&Statistics>) {
        for i in 0..statistics.len() {
            statistics[i].iterations += 1;
            statistics[i].damage_done += added[i].damage_done;
            statistics[i].damage_taken += added[i].damage_taken;
            statistics[i].healing_done += added[i].healing_done;
            statistics[i].healing_taken += added[i].healing_taken;
            statistics[i].shielding_done += added[i].shielding_done;
            statistics[i].shielding_taken += added[i].shielding_taken;

            statistics[i].damage_done_sq += added[i].damage_done * added[i].damage_done;
            statistics[i].damage_taken_sq += added[i].damage_taken * added[i].damage_taken;
            statistics[i].healing_done_sq += added[i].healing_done * added[i].healing_done;
            statistics[i].healing_taken_sq += added[i].healing_taken * added[i].healing_taken;
            statistics[i].shielding_done_sq += added[i].shielding_done * added[i].shielding_done;
            statistics[i].shielding_taken_sq += added[i].shielding_taken * added[i].shielding_taken;
        }
    }

    pub fn print_results(&self) {
        let mut wins = 0;
        let mut losses = 0;
        let mut stalls = 0;
        for result in &self.results {
            if result.win {
                wins += 1;
            }
            else if result.loss {
                losses += 1;
            }
            else if result.stall {
                stalls += 1;
            }
        }
        println!("wins: {}, losses: {}, stalls: {}", wins, losses, stalls);
    }


    pub fn print_statistics(&self) {
        let mut index = 0;
        for hero in self.allies.iter() {
            println!("{}:", hero.name);
            self.statistics[index].print();
            index += 1;
        }
        for hero in self.enemies.iter() {
            println!("{}:", hero.name);
            self.statistics[index].print();
            index += 1;
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.iterations {
            let mut wave = Wave::new(self.allies, self.enemies);
            let result = wave.run();
            self.results.push(result);
            Self::add_statistics(&mut self.statistics, &wave.get_statistics());
        }
    }
}
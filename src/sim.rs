use std::collections::HashMap;
use itertools::Itertools;

use crate::hero::Hero;
use crate::wave::Wave;
use crate::wave::Result;

pub struct Sim<'a> {
    allies: &'a Vec<&'a Hero>,
    enemies: &'a Vec<&'a Hero>,
    iterations: u32,
    results : Vec<Result>,
    statistics: Vec<SimStatistics>,
}

pub struct SimStatistics {
    pub iterations: u32,
    pub hm: HashMap<String, u32>,
    pub hm_sq: HashMap<String, u32>,
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
    pub fn print(&self) {
        // loop keys in hashmap hm and print
        for key in self.hm.keys().sorted() {
            let value = self.hm[key];
            println!("\t {}: {} +- {}", key, get_mean(value, self.iterations), get_standard_deviation(value, self.hm_sq[key], self.iterations));
        }
    }
}


impl Sim<'_> {
    pub fn new<'a>(allies: &'a Vec<&'a Hero>, enemies : &'a Vec<&'a Hero> , iterations: u32) -> Sim<'a> {
        // create statistcs vector with one entry per hero
        let mut statistics = Vec::new();
        for _ in 0..(allies.len() + enemies.len()) {
            statistics.push(SimStatistics {
                iterations :0 ,
                hm : HashMap::new(),
                hm_sq : HashMap::new(),
            });
        }
        Sim {
            allies: allies,
            enemies: enemies,
            iterations : iterations,
            results : Vec::new(),
            statistics : statistics,
        }
    }

    pub fn add_statistics( statistics : &mut Vec<SimStatistics>,  added : &Vec<&HashMap<String,u32>>) {
        for i in 0..statistics.len() {
            statistics[i].iterations += 1;
            for (key, value) in added[i] {
                *statistics[i].hm.entry(key.clone()).or_insert(0) += value;
                *statistics[i].hm_sq.entry(key.clone()).or_insert(0) += value*value;
            }
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
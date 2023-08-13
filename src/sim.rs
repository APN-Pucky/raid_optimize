use std::collections::HashMap;
use itertools::Itertools;

use rayon::prelude::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use crate::hero::Hero;
use crate::wave::Wave;
use crate::wave::Result;



pub fn get_mean(sum : f32, N: u32) -> f32 {
    (sum as f32) / N as f32
}

pub fn get_standard_deviation(sum : f32, sum_sq:f32, N: u32) -> f32 {
    ((sum_sq as f32 - sum as f32 * sum as f32 / N as f32) / N as f32).sqrt()
}

pub fn get_mean_and_standard_deviation(sum : f32, sum_sq:f32, N: u32) -> (f32, f32) {
    (get_mean(sum, N), get_standard_deviation(sum, sum_sq, N))
}

pub struct Sim<'a> {
    allies: &'a Vec<&'a Hero>,
    enemies: &'a Vec<&'a Hero>,
    iterations: u32,
    //results : Vec<Result>,
    result : CombinedResult,
}


pub struct CombinedResult {
    pub iterations: u32,
    pub wins: u32,
    pub losses: u32,
    pub stalls: u32,
    pub statistics: Vec<CombinedStatistics>,
}

pub struct CombinedStatistics {
    pub hm: HashMap<String, f32>,
    pub hm_sq: HashMap<String, f32>,
}

impl  CombinedResult {
    pub fn new(results : &Vec<Result>) -> CombinedResult {
        let mut cr  = CombinedResult {
            iterations: 0,
            wins: 0,
            losses: 0,
            stalls: 0,
            statistics: Vec::new(),
        };
        for r in results {
            cr.add_result(&r);
        }
        cr
    }

    pub fn add_combined_result(result : &mut CombinedResult, added : &CombinedResult) {
        result.iterations += added.iterations;
        result.wins += added.wins;
        result.losses += added.losses;
        result.stalls += added.stalls;
        Self::add_combined_statistics(&mut result.statistics, &added.statistics);
    }

    pub fn add_result(self: &mut CombinedResult, added : &Result) {
        self.iterations += 1;
        if added.win {
            self.wins += 1;
        }
        else if added.loss {
            self.losses += 1;
        }
        else if added.stall {
            self.stalls += 1;
        }
        Self::add_statistics(&mut self.statistics, &added.statistics);
    }

    pub fn add_combined_statistics( statistics : &mut Vec<CombinedStatistics>,  added : &Vec<CombinedStatistics>) {
        if statistics.len() < added.len() {
            for i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm : HashMap::new(),
                    hm_sq : HashMap::new(),
                });
            }
        }
        for i in 0..statistics.len() {
            for (key, value) in &added[i].hm {
                *statistics[i].hm.entry(key.clone()).or_insert(0.0) += added[i].hm[key];
                *statistics[i].hm_sq.entry(key.clone()).or_insert(0.0) += added[i].hm_sq[key];
            }
        }
    }

    pub fn add_statistics( statistics : &mut Vec<CombinedStatistics>,  added : &Vec<HashMap<String,u32>>) {
        if statistics.len() < added.len() {
            for i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm : HashMap::new(),
                    hm_sq : HashMap::new(),
                });
            }
        }
        for i in 0..statistics.len() {
            for (key, value) in &added[i] {
                let v= *value as f32;
                *statistics[i].hm.entry(key.clone()).or_insert(0.0) += v;
                *statistics[i].hm_sq.entry(key.clone()).or_insert(0.0) += v*v;
            }
        }
    }

    pub fn print_statistics(&self, index : usize) {
        // loop keys in hashmap hm and print
        for key in self.statistics[index].hm.keys().sorted() {
            let value = self.statistics[index].hm[key];
            println!("\t {}: {} +- {}", key, get_mean(value, self.iterations), get_standard_deviation(value, self.statistics[index].hm_sq[key], self.iterations));
        }
    }
    
}


impl Sim<'_> {
    pub fn new<'a>(allies: &'a Vec<&'a Hero>, enemies : &'a Vec<&'a Hero> , iterations: u32) -> Sim<'a> {
        // create statistcs vector with one entry per hero
        Sim {
            allies: allies,
            enemies: enemies,
            iterations : iterations,
            result : CombinedResult {
                iterations: 0,
                wins: 0,
                losses: 0,
                stalls: 0,
                statistics: Vec::new(),
            },
        }
    }


    pub fn print_results(&self) {
        println!("wins: {}, losses: {}, stalls: {}", self.result.wins, self.result.losses, self.result.stalls);
    }


    pub fn print_statistics(&self) {
        let mut index = 0;
        for hero in self.allies.iter() {
            println!("{}:", hero.name);
            self.result.print_statistics(index);
            index += 1;
        }
        for hero in self.enemies.iter() {
            println!("{}:", hero.name);
            self.result.print_statistics(index);
            index += 1;
        }
    }

    pub fn run(&mut self , threads : u32) {
        let vecit : Vec<u32> = (0..threads).collect::<Vec<_>>();
        let iter = self.iterations / threads;
        let bar = ProgressBar::new(self.iterations as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            )
            .unwrap(),
        );
        let results : Vec<CombinedResult> = vecit.par_iter().map(|i| {
            let mut cr = CombinedResult {
                iterations: 0,
                wins: 0,
                losses: 0,
                stalls: 0,
                statistics: Vec::new(),
            };
            for x in 0..iter {
                let mut wave = Wave::new(self.allies, self.enemies);
                cr.add_result(&wave.run());
                if x % 10000 == 0 {
                    bar.inc(10000);
                }
            }
            cr
        }).collect::<Vec<_>>();
        self.result = results.iter().fold(CombinedResult::new(&Vec::new()), |mut acc, x| {
            CombinedResult::add_combined_result(&mut acc, x);
            acc
        });
        //for _ in 0..self.iterations {
        //    let mut wave = Wave::new(self.allies, self.enemies);
        //    let result = wave.run();
        //    self.results.push(result);
        //    Self::add_statistics(&mut self.statistics, &wave.get_statistics());
        //}
    }
}
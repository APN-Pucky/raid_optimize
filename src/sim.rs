use enum_map::EnumMap;

use rayon::prelude::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use prettytable::Table;
use prettytable::Cell;
use prettytable::Row;

use crate::hero::Hero;
use crate::hero::instance::Instance;
use crate::hero::stat::Stat;
use crate::player::ManualPlayer;
use crate::player::Player;
use crate::player::RandomPlayer;
use crate::wave::InstanceRef;
use crate::wave::Wave;
use crate::wave::Result;



pub fn get_mean(sum : f64, n: u64) -> f64 {
    (sum ) / n as f64
}

pub fn get_standard_deviation(sum : f64, sum_sq:f64, n: u64) -> f64 {
    ((sum_sq - sum  * sum  / n as f64) / n as f64).sqrt()
}

pub fn get_mean_and_standard_deviation(sum : f64, sum_sq:f64, n: u64) -> (f64, f64) {
    (get_mean(sum, n), get_standard_deviation(sum, sum_sq, n))
}

pub struct Sim<'a> {
    allies: &'a Vec<&'a Hero>,
    enemies: &'a Vec<&'a Hero>,
    manual_ally : bool,
    manual_enemy: bool,
    iterations: u64,
    //results : Vec<Result>,
    result : CombinedResult,
}


pub struct CombinedResult {
    pub iterations: u64,
    pub wins: u32,
    pub losses: u32,
    pub stalls: u32,
    pub statistics: Vec<CombinedStatistics>,
}

pub struct CombinedStatistics {
    pub hm: EnumMap<Stat, f64>,
    pub hm_sq: EnumMap<Stat, f64>,
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
            cr.add_result(r);
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
            for _i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm : EnumMap::default(),
                    hm_sq : EnumMap::default(),
                });
            }
        }
        for (s,a) in statistics.iter_mut().zip(added.iter()) {
            for (key, _value) in a.hm.iter() {
                s.hm[key] += a.hm[key];
                s.hm_sq[key]+= a.hm_sq[key];
            }
        }
    }

    pub fn add_statistics( statistics : &mut Vec<CombinedStatistics>,  added : &Vec<EnumMap<Stat,f32>>) {
        if statistics.len() < added.len() {
            for _i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm : EnumMap::default(),
                    hm_sq : EnumMap::default(),
                });
            }
        }
        for (s,a) in statistics.iter_mut().zip(added.iter()) {
            for (key, value) in a {
                let v= *value as f64;
                s.hm[key] += v;
                s.hm_sq[key] += v*v;
            }
        }
    }

    pub fn print_statistics(&self, index : usize) {
        for (key,value) in self.statistics[index].hm.iter() {
            //let value = self.statistics[index].hm[key];
            println!("\t {}: {:.2} +- {:.2}", key, 
                get_mean(*value, self.iterations), 
                get_standard_deviation(*value, self.statistics[index].hm_sq[key], 
                self.iterations));
        }
    }

    pub fn get_mean(&self, index: usize,key: Stat) -> f64 {
        let hm =  &self.statistics[index].hm;
        // hm has key? else return 0.0
        get_mean(hm[key], self.iterations)
    }

    pub fn get_std(&self, index: usize , key : Stat) -> f64 {
        let hm =  &self.statistics[index].hm;
        get_standard_deviation(hm[key], self.statistics[index].hm_sq[key], self.iterations)
    }
    
}


impl Sim<'_> {
    pub fn new<'a>(allies: &'a Vec<&'a Hero>, enemies : &'a Vec<&'a Hero>,manual_ally:bool,manual_enemy: bool , iterations: u64) -> Sim<'a> {
        // create statistcs vector with one entry per hero
        Sim {
            allies,
            enemies,
            manual_ally,
            manual_enemy,
            iterations,
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
        println!("win%:\t{:>6.2} ({} / {})", self.result.wins as f64 / self.result.iterations as f64*100., self.result.wins, self.result.iterations);
        println!("stall%:\t{:>6.2} ({} / {})", self.result.stalls as f64 / self.result.iterations as f64*100., self.result.stalls, self.result.iterations);
        println!("loss%:\t{:>6.2} ({} / {})", self.result.losses as f64 / self.result.iterations as f64*100., self.result.losses, self.result.iterations);
    }


    pub fn print_statistics(&self, bar : bool) {
        let barlen = 20.0;
        let mut atable = Table::new();
        let mut row = Vec::new();


        row.push(Cell::new("Allies"));
        for hero in self.allies.iter() {
            row.push(Cell::new(&hero.name));
        }
        atable.set_titles(Row::new(row));

        for (key,_v) in self.result.statistics[0].hm.iter() {
            let mut row = Vec::new();
            row.push(Cell::new(&format!("{}",key)));
            let mut index = 0;
            let mut max : f64 = 0.0;
            for _her in self.allies.iter() {
                let value =self.result.get_mean(index,key) ;
                if value > max {
                    max = value;
                }
                index += 1;
            }
            index = 0;
            for _her in self.allies.iter() {
                //let value = self.result.statistics[index].hm[key];
                let mean = self.result.get_mean(index, key);
                let std = self.result.get_std(index, key);
                let s: String = if bar {
                        "=".repeat((mean/max*barlen) as usize) + &" ".repeat(((max-mean)/max*barlen) as usize)
                    }
                    else {
                        format!("{:.2} +- {:.2}",mean, std)
                    };
                row.push(Cell::new(&s));
                index += 1;
            }
            atable.add_row(Row::new(row));
        }
        atable.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        atable.printstd();


        let mut etable = Table::new();
        row = Vec::new();
        row.push(Cell::new("Enemies"));
        for hero in self.enemies.iter() {
            // append to vec
            row.push(Cell::new(&hero.name));
        }
        etable.set_titles(Row::new(row));
        for (key,_value) in self.result.statistics[self.allies.len()].hm.iter() {
            let mut row = Vec::new();
            row.push(Cell::new(&format!("{}",key)));
            let mut index = self.allies.len();
            let mut max : f64 = 0.0;
            for _her in self.enemies.iter() {
                let value =self.result.get_mean(index,key) ;
                if value > max {
                    max = value;
                }
                index += 1;
            }
            index = self.allies.len();
            for _her in self.enemies.iter() {
                //let value = self.result.statistics[index].hm[key];
                let mean = self.result.get_mean(index, key);
                let std = self.result.get_std(index, key);
                let s : String = if bar {
                        "=".repeat((mean/max*barlen) as usize) + &" ".repeat(((max-mean)/max*barlen) as usize)
                    }
                    else {
                        format!("{:.2} +- {:.2}",mean, std)
                    };
                row.push(Cell::new(&s));
                index += 1;
            }
            etable.add_row(Row::new(row));
        }
        etable.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        etable.printstd();
    }


    pub fn run(&mut self , threads : u32,track_statistics:bool) {
        let vecit : Vec<u32> = (0..threads).collect::<Vec<_>>();
        let iter = self.iterations / (threads as u64) ;
        let bar = ProgressBar::new(self.iterations);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            )
            .unwrap(),
        );
        let results : Vec<CombinedResult> = vecit.par_iter().map(|_i| {
            let mut cr = CombinedResult {
                iterations: 0,
                wins: 0,
                losses: 0,
                stalls: 0,
                statistics: Vec::new(),
            };
            for x in 0..iter {
                let ap : Box<dyn Player> = if self.manual_ally {
                    Box::new(ManualPlayer{})
                }
                else {
                     Box::new(RandomPlayer{})
                };
            
                let ep : Box<dyn Player> = if self.manual_enemy {
                    Box::new(ManualPlayer{})
                }
                else {
                    Box::new(RandomPlayer{})
                };
                let mut id = 0;
                let mut a : Vec<Instance>= self.allies.iter().map(|h| {
                    id += 1;
                    Instance::new(h, id , InstanceRef { team:true, index: (id-1) as usize },track_statistics)
                }).collect();
                let mut e: Vec<Instance>= self.enemies.iter().map(|h| {
                    id += 1;
                    Instance::new(h, id, InstanceRef { team:false, index: (id-1-self.allies.len()as u32) as usize },track_statistics)
                }).collect();
                let mut wave = Wave::new(&mut a, &mut e,ap,ep,track_statistics);
                cr.add_result(&wave.run());
                if (x+1) % 100000 == 0 { // plus one because we start at 0 and want the score added after the iteration
                    bar.inc(100000);
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
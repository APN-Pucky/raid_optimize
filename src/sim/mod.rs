use enum_map::EnumMap;

use rayon::prelude::*;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use prettytable::Table;
use prettytable::Cell;
use prettytable::Row;

use crate::data::hero::Hero;
use crate::data::instance::Instance;
use crate::error;
use crate::player::ManualPlayer;
use crate::player::Player;
use crate::player::RandomPlayer;
use crate::wave::Wave;
use crate::wave::Result;
use crate::wave::stat::Stat;
use tokio::sync::mpsc::UnboundedSender;

use self::args::Args;
use self::results::CombinedResult;

pub mod args;
pub mod results;





#[derive(Debug,Clone)]
pub struct Sim {
    args : Args,
    //results : Vec<Result>,
    //result : CombinedResult,
}


impl Sim {
    pub fn new( args : Args
        //allies: &'a Vec<&'a Hero>, enemies : &'a Vec<&'a Hero>,manual_ally:bool,manual_enemy: bool , iterations: u64
    ) -> Sim {
        // create statistcs vector with one entry per hero
        Sim {
            args,
        }
    }

    /* 
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
        for hero in self.args.allies.iter() {
            row.push(Cell::new(&hero.name));
        }
        atable.set_titles(Row::new(row));

        for (key,_v) in self.result.statistics[0].hm.iter() {
            let mut row = Vec::new();
            row.push(Cell::new(&format!("{}",key)));
            let mut index = 0;
            let mut max : f64 = 0.0;
            for _her in self.args.allies.iter() {
                let value =self.result.get_mean(index,key) ;
                if value > max {
                    max = value;
                }
                index += 1;
            }
            index = 0;
            for _her in self.args.allies.iter() {
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
        for hero in self.args.enemies.iter() {
            // append to vec
            row.push(Cell::new(&hero.name));
        }
        etable.set_titles(Row::new(row));
        for (key,_value) in self.result.statistics[self.args.allies.len()].hm.iter() {
            let mut row = Vec::new();
            row.push(Cell::new(&format!("{}",key)));
            let mut index = self.args.allies.len();
            let mut max : f64 = 0.0;
            for _her in self.args.enemies.iter() {
                let value =self.result.get_mean(index,key) ;
                if value > max {
                    max = value;
                }
                index += 1;
            }
            index = self.args.allies.len();
            for _her in self.args.enemies.iter() {
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
    */


    pub fn run(&self , tx :UnboundedSender<CombinedResult> ) {
        let vecit : Vec<u32> = (0..self.args.threads as u32).collect::<Vec<_>>();
        let iter = self.args.iterations / (self.args.threads as u64) ;
        //let bar = ProgressBar::new(self.args.iterations);
        //bar.set_style(
        //    ProgressStyle::with_template(
        //        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        //    )
        //    .unwrap(),
        //);
        vecit.par_iter().map(|_i| {
            let mut cr = CombinedResult::default();
            // TODO refactor
            let ap : Box<dyn Player> = if self.args.manual_ally {
                Box::new(ManualPlayer{team_index:0})
            }
            else {
                 Box::new(RandomPlayer{team_index:0})
            };
            
            let ep : Box<dyn Player> = if self.args.manual_enemy {
                Box::new(ManualPlayer{team_index:1})
            }
            else {
                Box::new(RandomPlayer{team_index:1})
            };
            let mut id = 0;
            let mut a : Vec<Instance>= self.args.allies.iter().map(|h| {
                id += 1;
                Instance::new(h, id , 0,0)
            }).collect();
            let mut e: Vec<Instance>= self.args.enemies.iter().map(|h| {
                id += 1;
                Instance::new(h, id, 0,1)
            }).collect();

            let mut instance = a.iter_mut().chain(e.iter_mut()).collect::<Vec<_>>();
            let mut players : Vec<Box<dyn Player>> = vec![ap,ep]; 
            let mut wave = Wave::new(&mut instance, &mut players ,!self.args.no_stats);
            for x in 0..iter {
                cr.add_result(&wave.run());
                wave.reset();
                if (x+1) % 1000 == 0 { // plus one because we start at 0 and want the score added after the iteration
                    match tx.send(cr) {
                        Ok(_) => {},
                        Err(e) => {
                            return ();
                        }
                    }
                    cr = CombinedResult::default();
                    //bar.inc(1000);
                }
            }
            ()
        }).collect::<Vec<_>>();

        //results.iter().fold(CombinedResult::new(&Vec::new()), |mut acc, x| {
        //    CombinedResult::add_combined_result(&mut acc, x);
        //    acc
        //})
        //for _ in 0..self.iterations {
        //    let mut wave = Wave::new(self.allies, self.enemies);
        //    let result = wave.run();
        //    self.results.push(result);
        //    Self::add_statistics(&mut self.statistics, &wave.get_statistics());
        //}
    }
}
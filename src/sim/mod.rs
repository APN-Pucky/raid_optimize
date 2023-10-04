

use rayon::prelude::*;








use crate::data::instance::Instance;

use crate::player::ManualPlayer;
use crate::player::Player;
use crate::player::RandomPlayer;
use crate::wave::Wave;


use tokio::sync::mpsc::UnboundedSender;

use self::args::Args;
use self::results::CombinedResult;

pub mod args;
pub mod results;
#[cfg(test)]
mod tests;





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

    pub fn run(&self , tx :UnboundedSender<CombinedResult> )   {
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
            let mut wave = Wave::new(&mut instance, &mut players ,self.args.stats);
            for x in 0..iter {
                cr.add_result(&wave.run());
                wave.reset();
                if (x+1) % 1000 == 0 { // plus one because we start at 0 and want the score added after the iteration
                    match tx.send(cr) {
                        Ok(_) => {},
                        Err(_e) => {
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
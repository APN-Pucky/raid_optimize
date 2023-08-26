use enum_map::EnumMap;
use rand::Rng;
use std::convert::TryInto;

pub mod damage;
pub mod inflict;
pub mod shield;
pub mod restore;
pub mod turn_meter;
pub mod action;
pub mod stat;
pub mod begin;
pub mod attributes;
pub mod skill;
pub mod passive;
pub mod effect;
pub mod dot;
pub mod faction;
pub mod print;

use crate::hero::Hero;
use crate::hero::effect::Effect;
use crate::hero::effects::Effects;
use crate::hero::instance::Instance;
use crate::hero::passive::Passive;
use crate::hero::skill::{Skill, get_targets, execute_skill};
use crate::player::Player;
use crate::{debug, indent, info};

use self::stat::{Stat, Statistics};


pub type TeamIndex = usize;
pub type InstanceIndex = usize;
// this serves as a ECS system
pub struct Wave<'a, const LEN: usize > {
    //pub allies: &'a mut Vec<Instance<'a>>, // should this be position dependent?
    //pub enemies: &'a mut Vec<Instance<'a>>,

    //pub instances : &'a mut Vec<Instance<'a>>,
    pub heroes :  [&'a Hero; LEN], 
    pub teams : [TeamIndex;LEN],
    pub players : &'a mut Vec<Box<dyn Player<LEN>>>, //TODO make this also generic 2!
    pub shields: [Vec<(f32,u32)>; LEN], // (shield_value, turns)
    pub effects : [Effects;LEN], 
    pub statistics: [Statistics;LEN],
    pub turn_meter : [f32;LEN],
    pub cooldowns : [Vec<u32>;LEN],
    pub health : [f32;LEN],
    //pub ally_player : Box<dyn Player>,
    //pub enemy_player : Box<dyn Player>,
    turns: u32,
    turn_limit: u32,
    turn_meter_threshold : f32,
    track_statistics: bool,
    len: usize,
}

pub struct Result {
    pub win: bool,
    pub loss: bool,
    pub stall : bool,
    pub statistics: Vec<EnumMap<Stat, f32>>,
}

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn new<'a>(instances: &'a mut  [&mut Instance<'a>;LEN], players:&'a mut Vec<Box<dyn Player<LEN>>>, track_statistics : bool) -> Wave<'a,LEN>  {
        // ensure right instance indices
        instances.iter_mut().enumerate().for_each(|(i,a)| a.index = i);
        let heroes : [&Hero; LEN] = instances.iter().map(|i| i.hero).collect::<Vec<_>>().try_into().unwrap();
        let teams = instances.iter().map(|i| i.team).collect::<Vec<_>>().try_into().unwrap();
        let statistics = instances.iter().map(|_| Statistics::new()).collect::<Vec<_>>().try_into().unwrap();
        let effects = instances.iter().map(|_| Effects::new()).collect::<Vec<_>>().try_into().unwrap();
        let turn_meter = instances.iter().map(|i| i.turn_meter).collect::<Vec<_>>().try_into().unwrap();
        let health = instances.iter().map(|i| i.health).collect::<Vec<_>>().try_into().unwrap();
        let mut cooldowns : [Vec<u32>;LEN] = instances.iter().map(|_| Vec::new()).collect::<Vec<_>>().try_into().unwrap();
        let shields = instances.iter().map(|_| Vec::new()).collect::<Vec<_>>().try_into().unwrap();
        // set the values of the cooldowns from the Instances
        for i in 0..LEN {
            for j in 0..instances[i].cooldowns.len() {
                cooldowns[i].push(instances[i].cooldowns[j]);
            }
        }
        // transform instances into ECS
        Wave {
            heroes,
            teams ,
            //instances.iter().map(|i| i.team).collect(),
            players,
            statistics ,
            shields ,
            effects ,
            cooldowns ,
            turn_meter ,
            health  ,
            turns: 0,
            turn_limit: 300,
            turn_meter_threshold:  1000.0 ,
            track_statistics,
            len : instances.len(),
        }
    }

    pub fn reset(&mut self) {
        for i in 0..LEN {
            self.statistics[i].clear();
            self.cooldowns[i].iter_mut().for_each(|c| *c = 0);
            self.turn_meter[i] = 0.0;
            self.health[i] = self.get_max_health(i);
            self.shields[i].clear();
            self.effects[i].clear();
        }
    }

    #[inline]
    pub fn get_indices(&self) -> Vec<InstanceIndex>
    {
        (0..LEN).collect::<Vec<_>>()
    }

    pub fn get_statistics(&self) -> Vec<EnumMap<Stat,f32>> {
        //self.allies.iter().chain(self.enemies.iter()).map(|i| i.statistics).collect()
        (0..LEN)
            .map(|i| self.statistics[i].sts)
            .collect()
    }


    pub fn get_player_of_instance(&self, ii: InstanceIndex) -> &dyn Player<LEN> {
        &*self.players[self.teams[ii]]
    }

    pub fn get_player_of_team(&self, team : TeamIndex) -> &dyn Player<LEN> {
        &*self.players[team]
    }

    pub fn get_enemies_indices(&self, actor : InstanceIndex) -> Vec<InstanceIndex> {
        (0..LEN)
            .filter(|&i| self.teams[i] != self.teams[actor])
            .collect()
    }

    pub fn get_ally_indices(&self, actor : InstanceIndex) -> Vec<InstanceIndex> {
        (0..LEN)
            .filter(|&i| self.teams[i] == self.teams[actor])
            .collect()
    }

    pub fn find_actor_index(&self) -> Option<InstanceIndex> {
        (0..LEN)
            // get those alive
            .filter(|&a| self.is_alive(a))
            // get those with enough turn meter
            .filter(|&a| self.get_turn_meter(a) >= self.turn_meter_threshold)
            // get instance with highest speed
            //.reduce( |a, b| if a.get_speed() > b.get_speed() {a} else {b})
            .max_by(|a,b| self.get_speed(*a).partial_cmp(&self.get_speed(*b)).unwrap())
    }

    pub fn run(& mut self) -> Result {
        self.begin();
        loop {
            self.log_info();
            self.progress_all_turn_meters();
            
            if let Some(ir) = self.find_actor_index() {
                self.act(ir);
                self.turns += 1;
            }
            else {
                log::debug!("Nobody acts");
            }

            // game over
            // TODO hard coded team indices... of 0
            let win = self.get_enemies_indices(0).iter().all(|&e| !self.is_alive(e));
            let loss = self.get_ally_indices(0).iter().all(|&a| !self.is_alive(a));
            let mut stall = self.turns >= self.turn_limit;
            if win || loss || stall {
                if win {
                    log::debug!("Win");
                }
                if loss {
                    log::debug!("Loss");
                }
                if stall {
                    log::debug!("Stall");
                }
                if win && stall || loss && stall {
                    // last turn victory excludes stall
                    stall = false;
                }
                if win && loss {
                    println!("Turn: {}", self.turns);
                    self.print_all();
                    panic!("Inconsistent result win: {}, loss: {}, stall: {}", win,loss,stall);
                }
                return Result {
                    win: win,
                    loss: loss,
                    stall: self.turns >= self.turn_limit,
                    statistics: self.get_statistics(),
                }
            }
        }
        
    }
}
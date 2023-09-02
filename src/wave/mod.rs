use enum_map::EnumMap;
use rand::Rng;
use strum::EnumProperty;
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
pub mod skills;
pub mod passives;
pub mod effect;
pub mod dot;
pub mod faction;
pub mod print;
pub mod cleanse;

use crate::data::hero::Hero;
use crate::data::effect::Effect;
use crate::data::effects::Effects;
use crate::data::faction::Faction;
use crate::data::instance::Instance;
use crate::data::passive::Passive;
use crate::data::skill::{Skill, get_targets};
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
    pub shields: [Vec<(f32,u32,InstanceIndex)>; LEN], // (shield_value, turns)
    pub effects : [Effects;LEN], 
    pub statistics: [Statistics;LEN],
    pub turn_meter : [f32;LEN],
    pub cooldowns : [Vec<u32>;LEN],
    pub health : [f32;LEN],
    pub bonds_counter : [u32;LEN],
    pub team_bonds : Vec<EnumMap<Faction,f32>>,
    //pub ally_player : Box<dyn Player>,
    //pub enemy_player : Box<dyn Player>,
    pub team_acted : Vec<bool>,
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
        let bonds  = players.iter().map(|p| EnumMap::default()).collect::<Vec<_>>().try_into().unwrap();
        let bonds_counter = instances.iter().map(|_| 0).collect::<Vec<_>>().try_into().unwrap();
        // set the values of the cooldowns from the Instances
        for i in 0..LEN {
            for j in 0..instances[i].cooldowns.len() {
                cooldowns[i].push(instances[i].cooldowns[j]);
            }
        }
        let team_acted = vec![false;players.len()];
        // transform instances into ECS
        let mut w = Wave {
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
            team_acted,
            len : instances.len(),
            team_bonds : bonds,
            bonds_counter : bonds_counter,
        };
        w.set_bonds();
        w
    }


    pub fn reset(&mut self) {
        self.turns = 0;
        for i in 0..self.players.len() {
            self.team_acted[i] = false;
        }
        for i in 0..LEN {
            self.statistics[i].clear();
            self.cooldowns[i].iter_mut().for_each(|c| *c = 0);
            self.turn_meter[i] = 0.0;
            self.health[i] = self.get_max_health(i);
            self.shields[i].clear();
            self.effects[i].clear();
            self.bonds_counter[i] = 0;
        }
        self.set_bonds(); //  no change expecte
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

    pub fn revive(&mut self, actor: InstanceIndex, target: InstanceIndex, health_max_ratio: f32, health_abs : f32) {
        debug!("{} revives {}", self.name(actor), self.name(target));
        indent!({
            let ratio = self.get_revive_extra_hp_ratio(target);
            self.health[target] = self.get_max_health(target).min(ratio*(health_max_ratio * self.get_max_health(target) + health_abs ));
            self.add_stat(actor, Stat::Revives, 1.0);
            self.add_stat(target, Stat::Revived, 1.0);
        })

    }


    pub fn get_player_of_instance(&self, ii: InstanceIndex) -> &dyn Player<LEN> {
        &*self.players[self.teams[ii]]
    }

    pub fn get_player_of_team(&self, team : TeamIndex) -> &dyn Player<LEN> {
        &*self.players[team]
    }

    pub fn get_team_indices(&self, team : TeamIndex) -> Vec<InstanceIndex> {
        (0..LEN)
            .filter(|&i| self.teams[i] == team)
            .collect()
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
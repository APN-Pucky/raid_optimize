use rand::Rng;
use std::collections::HashMap;

use crate::hero::Hero;
use crate::hero::effect::Effect;
use crate::hero::instance::Instance;
use crate::hero::skill::{Skill, get_targets, execute_skill};
use crate::player::Player;

#[derive(Debug,Copy,Clone)]
pub struct InstanceRef {
    pub team: bool,
    pub index: usize,
}

pub struct Wave {
    pub allies: Vec<Instance>, // should this be position dependent?
    pub enemies: Vec<Instance>,
    pub ally_player : Box<dyn Player>,
    pub enemy_player : Box<dyn Player>,
    turns: u32,
    turn_limit: u32,
    initiative_threshold: u32,
}

pub struct Result {
    pub win: bool,
    pub loss: bool,
    pub stall : bool,
    pub statistics: Vec<HashMap<String, u32>>,
}

impl Wave {
    pub fn new(allies: & Vec<&Hero>, enemies : & Vec<&Hero>) -> Wave {
        let mut id = 0;
        let a= allies.iter().map(|h| {
            id += 1;
            Instance::new(h, id , InstanceRef { team:true, index: (id-1) as usize })
        }).collect();
        let e= enemies.iter().map(|h| {
            id += 1;
            Instance::new(h, id, InstanceRef { team:false, index: (id-1-allies.len()as u32) as usize })
        }).collect();
        Wave {
            allies:a,
            enemies:e,
            ally_player: Box::new(crate::player::RandomPlayer{}),
            enemy_player: Box::new(crate::player::RandomPlayer{}),
            turns: 0,
            turn_limit: 300,
            initiative_threshold: 1000,
        }
    }

    pub fn get_statistics(&self) -> Vec<HashMap<String,u32>> {
        self.allies.iter().chain(self.enemies.iter()).map(|i| i.copy_statistics()).collect()
    }

    pub fn get_instance(&self, actor : InstanceRef) -> &Instance {
        if actor.team {
            &self.allies[actor.index]
        }
        else {
            &self.enemies[actor.index]
        }
    }

    pub fn get_instance_mut(&mut self, actor : InstanceRef) -> &mut Instance {
        if actor.team {
            &mut self.allies[actor.index]
        }
        else {
            &mut self.enemies[actor.index]
        }
    }

    pub fn get_instance_ref(&self, actor : &Instance) -> InstanceRef {
        match self.allies.iter().position(|a| *a == *actor) {
            Some(index) => InstanceRef{team: true, index},
            None => {
                let index = self.enemies.iter().position(|a| *a == *actor).unwrap();
                InstanceRef{team: false, index}
            }
        }
    }

    pub fn find_actor_index(&self) -> Option<InstanceRef> {
        // TODO look up speed value in case of equal initiative
        let mut index = 0;
        let mut found = false;
        let mut max_turn_meter = 0;
        let mut max_index = 0;
        for actor in self.allies.iter() {
            if actor.get_turn_meter() > max_turn_meter {
                max_turn_meter = actor.get_turn_meter();
                max_index = index;
                found = true;
            }
            index += 1;
        }
        index = 0;
        for actor in self.enemies.iter() {
            if actor.get_turn_meter() > max_turn_meter {
                max_turn_meter = actor.get_turn_meter();
                max_index = index;
                found = false;
            }
            index += 1;
        }
        if max_turn_meter >= self.initiative_threshold {
            log::debug!("{} acts", self.get_instance(InstanceRef{team : found,index: max_index}));
            Some(InstanceRef{team : found,index: max_index})
        } else {
            log::debug!("Nobody acts");
            None
        }
    }

    pub fn get_team(&self, actor : InstanceRef) -> &Vec<Instance> {
        if actor.team {
            &self.allies
        }
        else {
            &self.enemies
        }
    }

    pub fn get_enemy_team(&self, actor : &InstanceRef) -> &Vec<Instance> {
        if !actor.team {
            &self.allies
        }
        else {
            &self.enemies
        }
    }

    pub fn choose_target(&self, actor : &InstanceRef) -> Option<InstanceRef> {
        let team = self.get_enemy_team(actor);
        //opponents.iter().filter(|i| i.is_alive()).collect::<Vec<&mut Instance>>().choose_mut(&mut rand::thread_rng())
        let mut ids = Vec::new();
        let mut index = 0;
        for target in team.iter() {
            if target.is_alive()  {
                ids.push(index);
            }
            index += 1;
        }
        // pick random index from ids
        if ids.len() == 0 {
            return None;
        }
        else {
            let mut rng = rand::thread_rng();
            let ri = rng.gen_range(0..ids.len());
            Some(InstanceRef{team: !actor.team , index:ids[ri]})
        }
    }

    pub fn increase_initiatives(&mut self) {
        // get the time needed for one to reach threshold
        let mut min : f32 = self.allies.iter().chain(self.enemies.iter()).map(|a| (self.initiative_threshold - a.get_turn_meter() ) as f32 /(a.get_speed() as f32)).fold(f32::INFINITY, |a, b| a.min(b));
        if min < 0.0 {
            min = 0.0;
        }
        self.allies.iter_mut().for_each(|a| a.increase_turn_meter(min));
        self.enemies.iter_mut().for_each(|a| a.increase_turn_meter(min));
    }

    pub fn before_action(&mut self, actor : InstanceRef) {
        let mut a; 
        let mut e;
        if actor.team {
            a = &mut self.allies[actor.index];
            e = &mut self.enemies;
        }{
            a = &mut self.enemies[actor.index];
            e = &mut self.allies;
        }
        let n = a.effects.get(Effect::Bleed);
        if n > 0 {
            let b : &Vec<(u32,InstanceRef)> = a.effects.hm.get(&Effect::Bleed).unwrap();
            // get last element of b
            let nn: &InstanceRef = &b.last().unwrap().1;
            let dmg_vec = vec![0.30,0.50,0.70,0.90,1.05,1.20,1.35,1.45,1.55,1.65];
            let bleed_dmg = (e[nn.index].get_attack_damage()as f32 * dmg_vec[n as usize]) as u32;
            a.take_bleed_damage(bleed_dmg);
        }
        a.reduce_cooldowns();
    }

    pub fn after_action(&mut self, actor :InstanceRef) {
        let mut a; 
        let mut e;
        if actor.team {
            a = &mut self.allies[actor.index];
            e = &mut self.enemies;
        }{
            a = &mut self.enemies[actor.index];
            e = &mut self.allies;
        }
        a.set_turn_meter(0);
    }

    pub fn act(&mut self, actor : InstanceRef) {
        self.before_action(actor);
        // choose action
        let instance  = self.get_instance(actor);
        let skills : Vec<Skill> = instance.get_active_skills();
        log::debug!("{} has active skills {:?}", instance, skills);
        let skill :Skill; 
        if actor.team {
            skill = self.ally_player.pick_skill(self, actor, skills);
        }
        else {
            skill = self.enemy_player.pick_skill(self, actor, skills);
        }
        // get targets
        match get_targets(skill, &actor, self) {
            Some(ts) => {
                let target : InstanceRef;
                if actor.team {
                    target = self.ally_player.pick_target(self, actor, skill, ts);
                }
                else {
                    target = self.enemy_player.pick_target(self, actor, skill,ts);
                }
                // apply skill
                execute_skill(skill, &actor, &target, self);
            },
            None => {
                // TODO maybe not even provide this option as active skill
                log::debug!("{} has no valid targets for {}", instance, skill);
                return;
            },
        }
        // finish
        self.after_action(actor);

    }

    pub fn print(&self) {
        log::info!("Turn: {}", self.turns); 
        log::info!("Allies:");
        for a in self.allies.iter() {
            log::info!("{}", a);
        }
        log::info!("Enemies:");
        for e in self.enemies.iter() {
            log::info!("{}", e);
        }
    }

    pub fn run(& mut self) -> Result {
        loop {
            self.print();
            self.increase_initiatives();
            match self.find_actor_index() {
                Some(ir) => {
                    self.act(ir);
                    self.turns += 1;
                },
                None => {},
            }

            // game over
            let win = self.enemies.iter().all(|e| !e.is_alive());
            let loss = self.allies.iter().all(|a| !a.is_alive());
            let stall = self.turns >= self.turn_limit;
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
                if win && loss || win && stall || loss && stall {
                    panic!("Inconsistent result");
                }
                return Result {
                    win: self.enemies.iter().all(|e| !e.is_alive()),
                    loss: self.allies.iter().all(|a| !a.is_alive()),
                    stall: self.turns >= self.turn_limit,
                    statistics: self.get_statistics(),
                }
            }
        }
        
    }
}
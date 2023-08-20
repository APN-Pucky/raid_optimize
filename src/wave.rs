use enum_map::EnumMap;
use rand::Rng;

use crate::hero::Hero;
use crate::hero::effect::Effect;
use crate::hero::instance::Instance;
use crate::hero::skill::{Skill, get_targets, execute_skill};
use crate::hero::stat::Stat;
use crate::player::Player;

pub const TURN_LIMIT : u32 = 300;
pub const TURN_METER_THRESHOLD : u32 = 1000;

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
    pub statistics: Vec<EnumMap<Stat, u32>>,
}

impl Wave {
    pub fn new(allies: & Vec<&Hero>, enemies : & Vec<&Hero> , ap:Box<dyn Player>, ep:Box<dyn Player> , track_statistics : bool) -> Wave {
        let mut id = 0;
        let a= allies.iter().map(|h| {
            id += 1;
            Instance::new(h, id , InstanceRef { team:true, index: (id-1) as usize },track_statistics)
        }).collect();
        let e= enemies.iter().map(|h| {
            id += 1;
            Instance::new(h, id, InstanceRef { team:false, index: (id-1-allies.len()as u32) as usize },track_statistics)
        }).collect();
        Wave {
            allies:a,
            enemies:e,
            ally_player: ap,
            enemy_player: ep,
            turns: 0,
            turn_limit: TURN_LIMIT,
            initiative_threshold: TURN_METER_THRESHOLD,
        }
    }

    pub fn get_statistics(&self) -> Vec<EnumMap<Stat,u32>> {
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
        let mut index = 0;
        let mut found = false;
        let mut max_turn_meter = 0;
        let mut max_index = 0;
        for actor in self.allies.iter() {
            if actor.is_alive() {
                if actor.get_turn_meter() > max_turn_meter {
                    max_turn_meter = actor.get_turn_meter();
                    max_index = index;
                    found = true;
                }
                if actor.get_turn_meter() == max_turn_meter {
                    // compare by speed stat
                    if actor.get_speed() > self.get_instance(InstanceRef{team : found,index: max_index}).get_speed() {
                        max_turn_meter = actor.get_turn_meter();
                        max_index = index;
                        found = true;
                    }
                }
            }
            index += 1;
        }
        index = 0;
        for actor in self.enemies.iter() {
            if actor.is_alive() {
                if actor.get_turn_meter() > max_turn_meter {
                    max_turn_meter = actor.get_turn_meter();
                    max_index = index;
                    found = false;
                }
                if actor.get_turn_meter() == max_turn_meter {
                    // compare by speed stat
                    if actor.get_speed() > self.get_instance(InstanceRef{team : found,index: max_index}).get_speed() {
                        max_turn_meter = actor.get_turn_meter();
                        max_index = index;
                        found = false;
                    }
                }
            }
            index += 1;
        }
        if max_turn_meter >= self.initiative_threshold {
            let ir = InstanceRef{team : found,index: max_index};
            log::debug!("{} acts {max_turn_meter}", self.get_instance(ir));
            Some(ir)
        } else {
            log::debug!("Nobody acts");
            None
        }
    }

    pub fn get_ally_team(&self, actor : &InstanceRef) -> &Vec<Instance> {
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

    pub fn inflict_team(&mut self, actor : &InstanceRef, effect : Effect, chance: f32, turns :u32) {
        if turns == 0 {
            return;
        }
        if actor.team {
            self.enemies.iter_mut().for_each(|a| a.get_inflicted(actor,effect,chance, turns));
        }
        else {
            self.allies.iter_mut().for_each(|a| a.get_inflicted(actor,effect, chance, turns));
        }
    }

    pub fn attack_team(&mut self, actor : &InstanceRef, damage : u32) {
        if actor.team {
            self.enemies.iter_mut().for_each(|a| a.take_damage(damage));
        }
        else {
            self.allies.iter_mut().for_each(|a| a.take_damage(damage));
        }
    }

    pub fn increase_turn_meter_team(&mut self, actor : &InstanceRef, increase_ratio : f32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.increase_turn_meter((increase_ratio * TURN_METER_THRESHOLD as f32 ) as u32));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.increase_turn_meter((increase_ratio * TURN_METER_THRESHOLD as f32) as u32));
        }
    }

    pub fn cleanse_team<F>(&mut self, actor : &InstanceRef, effect_closure: &F ,layers:u32) where F : Fn(Effect) -> bool {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.cleanse(effect_closure,layers));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.cleanse(effect_closure,layers));
        }
    }

    pub fn restore_max_hp_own_team(&mut self, actor : &InstanceRef, restore_max_hp: u32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.heal(restore_max_hp));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.heal(restore_max_hp));
        }
    }

    pub fn restore_max_hp_ratio_own_team(&mut self, actor : &InstanceRef, restore_max_hp_ratio: f32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.heal((a.get_max_health() as f32 * restore_max_hp_ratio ) as u32));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.heal((a.get_max_health() as f32 * restore_max_hp_ratio ) as u32));
        }
    }

    pub fn shield_team(&mut self, actor : &InstanceRef, shield_value:u32, shield_turns:u32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.add_shield(shield_value, shield_turns));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.add_shield(shield_value, shield_turns));
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

    pub fn progress_turn_meter(&mut self) {
        // get the time needed for one to reach threshold
        let mut min : f32 = self.allies.iter().chain(self.enemies.iter()).filter(|a| a.is_alive()).map(|a| (self.initiative_threshold - a.get_turn_meter() ) as f32 /(a.get_speed() as f32)).fold(f32::INFINITY, |a, b| a.min(b));
        if min < 0.0 {
            min = 0.0;
        }
        self.allies.iter_mut().for_each(|a| a.progress_turn_meter(min));
        self.enemies.iter_mut().for_each(|a| a.progress_turn_meter(min));
    }

    pub fn before_action(&mut self, actor : InstanceRef) {
        let a; 
        let e;
        if actor.team {
            a = &mut self.allies[actor.index];
            e = &mut self.enemies;
        }else {
            a = &mut self.enemies[actor.index];
            e = &mut self.allies;
        }
        log::debug!("before {} acts", a);
        // apply effects 
        // apply heal
        let n = a.effects.get(Effect::Heal);
        if n> 0 {
            let heal = (a.get_max_health() * 5 * n) /100;
            a.heal(heal);
        }
        // apply bleed
        let n = a.effects.get(Effect::Bleed);
        if n > 0 {
            let b : &Vec<(u32,InstanceRef)> = &a.effects.em[Effect::Bleed];
            // get inflictor
            let nn: &InstanceRef = &b.last().unwrap().1;
            let dmg_vec = vec![30,50,70,90,105,120,135,145,155,165];
            let bleed_dmg = (e[nn.index].get_attack_damage() * dmg_vec[n as usize]) /100;
            a.take_bleed_damage(bleed_dmg);
        }
        // apply HP burning
        let n = a.effects.get(Effect::HPBurning);
        if n > 0 {
            let b : &Vec<(u32,InstanceRef)> = &a.effects.em[Effect::HPBurning];
            // get inflictor
            let inflictor : &InstanceRef = &b.last().unwrap().1;
            let mut hp_burn_dmg = (a.get_max_health() * 8 * n) / 100;
            let max = 3*e[inflictor.index].get_max_health() /10;
            if hp_burn_dmg > max {
                hp_burn_dmg = max;
            }
            a.take_hp_burning_damage(hp_burn_dmg);
        }

        a.reduce_cooldowns();
    }

    pub fn after_action(&mut self, actor :InstanceRef) {
        let a; 
        //let e;
        if actor.team {
            a = &mut self.allies[actor.index];
            //e = &mut self.enemies;
        }else {
            a = &mut self.enemies[actor.index];
            //e = &mut self.allies;
        }
        log::debug!("after {} acts", a);
        a.set_turn_meter(0);
        a.reduce_effects();
        a.reduce_shields();
    }

    pub fn act(&mut self, actor : InstanceRef) {
        //
        if !self.get_instance(actor).is_alive() {
            log::debug!("{} is dead -> can't take turn", self.get_instance(actor));
            return;
        }
        self.before_action(actor);
        if !self.get_instance(actor).is_alive() {
            log::debug!("{} is dead now -> can't take turn", self.get_instance(actor));
            return;
        }
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

    pub fn info(&self) {
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

    pub fn print_allies(&self) {
        println!("Allies:");
        for a in self.allies.iter() {
            println!("{}", a);
        }
    }

    pub fn print_enemies(&self) {
        println!("Enemies:");
        for e in self.enemies.iter() {
            println!("{}", e);
        }
    }

    pub fn run(& mut self) -> Result {
        loop {
            self.info();
            self.progress_turn_meter();
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
                    panic!("Inconsistent result {},{},{}", win,loss,stall);
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
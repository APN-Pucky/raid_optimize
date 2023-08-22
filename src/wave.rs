use enum_map::EnumMap;
use rand::Rng;

use crate::hero::Hero;
use crate::hero::effect::Effect;
use crate::hero::instance::Instance;
use crate::hero::passive::Passive;
use crate::hero::skill::{Skill, get_targets, execute_skill};
use crate::hero::stat::Stat;
use crate::player::Player;
use crate::{debug, indent, info};

pub const TURN_LIMIT : u32 = 300;
pub const TURN_METER_THRESHOLD : f32 = 1000.0;


#[derive(Debug,Copy,Clone)]
pub struct InstanceRef {
    pub team: bool,
    pub index: usize,
}

pub struct Wave<'a> {
    pub allies: &'a mut Vec<Instance<'a>>, // should this be position dependent?
    pub enemies: &'a mut Vec<Instance<'a>>,
    pub ally_player : Box<dyn Player>,
    pub enemy_player : Box<dyn Player>,
    turns: u32,
    turn_limit: u32,
    initiative_threshold: f32,
}

pub struct Result {
    pub win: bool,
    pub loss: bool,
    pub stall : bool,
    pub statistics: Vec<EnumMap<Stat, f32>>,
}

impl<'a> Wave<'a> {

    pub fn get_active_skills(&self, actor : &InstanceRef) -> Vec<&'a Skill> {
        if actor.team {
            self.allies[actor.index].get_active_skills()
        }
        else {
            self.enemies[actor.index].get_active_skills()
        }
    }
}

impl Wave<'_> {
    pub fn new<'a>(a: &'a mut  Vec<Instance<'a>>, e: &'a mut Vec<Instance<'a>>, ap:Box<dyn Player>, ep:Box<dyn Player> , track_statistics : bool) -> Wave<'a> {
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

    pub fn get_statistics(&self) -> Vec<EnumMap<Stat,f32>> {
        self.allies.iter().chain(self.enemies.iter()).map(|i| i.statistics).collect()
    }


    pub fn get_instance(&self, actor : InstanceRef) -> &Instance {
        if actor.team {
            &self.allies[actor.index]
        }
        else {
            &self.enemies[actor.index]
        }
    }

    //pub fn get_instance_mut(&mut self, actor : InstanceRef) -> &mut Instance {
    //    if actor.team {
    //        &mut self.allies[actor.index]
    //    }
    //    else {
    //        &mut self.enemies[actor.index]
    //    }
    //}

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
        self.allies.iter().chain(self.enemies.iter())
            // get those alive
            .filter(|a| a.is_alive())
            // get those with enough turn meter
            .filter(|a| a.get_turn_meter() >= self.initiative_threshold)
            // get instance with highest speed
            //.reduce( |a, b| if a.get_speed() > b.get_speed() {a} else {b})
            .max_by(|a,b| a.get_speed().partial_cmp(&b.get_speed()).unwrap())
            // get instanceref
            .map(|a| a.iref)
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

    pub fn inflict_friendly(&mut self, actor : &InstanceRef,target: &InstanceRef,effect:Effect,chance:f32,turns:u32) {
        if actor.team {
            self.allies[target.index].get_inflicted(actor, effect,chance, turns);
        }
        else {
            self.enemies[target.index].get_inflicted(actor, effect,chance, turns);
        }
    }

    pub fn restore(&mut self, actor : &InstanceRef,target: &InstanceRef,health:f32) {
        if actor.team {
            self.allies[target.index].heal(health);
        }
        else {
            self.enemies[target.index].heal(health);
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

    pub fn attack_team(&mut self, actor : &InstanceRef, damage : f32) {
        if actor.team {
            self.enemies.iter_mut().for_each(|a| a.take_damage(damage));
        }
        else {
            self.allies.iter_mut().for_each(|a| a.take_damage(damage));
        }
    }

    pub fn increase_turn_meter_team(&mut self, actor : &InstanceRef, increase_ratio : f32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.increase_turn_meter((increase_ratio * TURN_METER_THRESHOLD  ) ));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.increase_turn_meter((increase_ratio * TURN_METER_THRESHOLD ) ));
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

    pub fn restore_max_hp_own_team(&mut self, actor : &InstanceRef, restore_max_hp: f32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.heal(restore_max_hp));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.heal(restore_max_hp));
        }
    }

    pub fn restore_max_hp_ratio_own_team(&mut self, actor : &InstanceRef, restore_max_hp_ratio: f32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.heal((a.get_max_health()  * restore_max_hp_ratio ) ));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.heal((a.get_max_health()  * restore_max_hp_ratio ) ));
        }
    }

    pub fn shield_team(&mut self, actor : &InstanceRef, shield_value:f32, shield_turns:u32) {
        if actor.team {
            self.allies.iter_mut().for_each(|a| a.add_shield(shield_value, shield_turns));
        }
        else {
            self.enemies.iter_mut().for_each(|a| a.add_shield(shield_value, shield_turns));
        }
    }

    pub fn choose_target(&self, actor : &InstanceRef) -> Option<InstanceRef> {
        let team = self.get_enemy_team(actor);
        let mut ids = Vec::new();
        for (index,target) in team.iter().enumerate() {
            if target.is_alive()  {
                ids.push(index);
            }
        }
        // pick random index from ids
        if ids.is_empty() {
            None
        }
        else {
            let mut rng = rand::thread_rng();
            let ri = rng.gen_range(0..ids.len());
            Some(InstanceRef{team: !actor.team , index:ids[ri]})
        }
    }

    pub fn progress_turn_meter(&mut self) {
        debug!("Progressing turn meter");
        indent!({
        // get the time needed for one to reach threshold
        let mut min : f32 = self.allies.iter().chain(self.enemies.iter())
            .filter(|a| a.is_alive())
            .map(|a| (self.initiative_threshold - a.get_turn_meter() )/(a.get_speed()))
            .min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap();
            //.reduce( |a, b| a.min(b)).unwrap();
        if min < 0.0 {
            min = 0.0;
        }
        self.allies.iter_mut()
            .chain(self.enemies.iter_mut())
            .for_each(|a| a.progress_turn_meter(min));
        })
    }

    pub fn before_action(&mut self, actor : InstanceRef) {
        let (a,e) = if actor.team {
                (&mut self.allies[actor.index], &mut self.enemies)
            }else {
                (&mut self.enemies[actor.index], &mut self.allies)
            };
        debug!("before {} acts", a);
        indent!({
        // apply effects 
        // apply heal
        let n = a.effects.get(Effect::Heal);
        if n> 0 {
            let heal = (a.get_max_health() * 0.05 * n as f32) ;
            a.heal(heal);
        }
        // apply bleed
        let n = a.effects.get(Effect::Bleed);
        if n > 0 {
            let b : &Vec<(u32,InstanceRef)> = &a.effects.em[Effect::Bleed];
            // get inflictor
            let nn: &InstanceRef = &b.last().unwrap().1;
            let dmg_vec = vec![0.30,0.50,0.70,0.90,1.05,1.20,1.35,1.45,1.55,1.65];
            let bleed_dmg = (e[nn.index].get_attack_damage() * dmg_vec[n as usize]) ;
            a.take_bleed_damage(bleed_dmg);
        }
        // apply HP burning
        let n = a.effects.get(Effect::HPBurning);
        if n > 0 {
            let b : &Vec<(u32,InstanceRef)> = &a.effects.em[Effect::HPBurning];
            // get inflictor
            let inflictor : &InstanceRef = &b.last().unwrap().1;
            let mut hp_burn_dmg : f32 = (a.get_max_health() * 0.08 * n as f32);
            let max = 0.3*e[inflictor.index].get_max_health();
            if hp_burn_dmg > max {
                hp_burn_dmg = max;
            }
            a.take_hp_burning_damage(hp_burn_dmg);
        }

        a.reduce_cooldowns();
        })
    }

    pub fn after_action(&mut self, actor :InstanceRef) {
        let a = if actor.team {
                &mut self.allies[actor.index]
                //e = &mut self.enemies;
            }else {
                &mut self.enemies[actor.index]
                //e = &mut self.allies;
            };
        debug!("after {} acts", a);
        indent!({
        a.set_turn_meter(0.0);
        a.reduce_effects();
        a.reduce_shields();
        })
    }


    pub fn act(&mut self, actor : InstanceRef) {
        debug!("{} acts", self.get_instance(actor));
        indent!({
        //
        if !self.get_instance(actor).is_alive() {
            debug!("{} is dead -> can't take turn", self.get_instance(actor));
            return;
        }
        self.before_action(actor);
        if !self.get_instance(actor).is_alive() {
            debug!("{} is dead now -> can't take turn", self.get_instance(actor));
            return;
        }
        // choose action
        let skills : Vec<&Skill> = self.get_active_skills(&actor);
        debug!("{} has active skills {:?}", self.get_instance(actor), skills);
        let skill :&Skill = if actor.team {
                self.ally_player.pick_skill(self, actor, &skills)
            }
            else {
                 self.enemy_player.pick_skill(self, actor, &skills)
            };
        debug!("{} chooses {}", self.get_instance(actor), skill);
        // get targets
        match get_targets(&skill, &actor, self) {
            Some(ts) => {
                let target : InstanceRef = if actor.team {
                        self.ally_player.pick_target(self, actor, &skill, &ts)
                    }
                    else {
                        self.enemy_player.pick_target(self, actor, &skill, &ts)
                    };
                // apply skill
                execute_skill(skill, &actor, &target, self);
            },
            None => {
                // TODO maybe not even provide this option as active skill
                debug!("{} has no valid targets for {}", self.get_instance(actor), skill);
                return;
            },
        }
        // finish
        self.after_action(actor);
        })
    }

    pub fn log_info(&self) {
        info!("Turn: {}", self.turns); 
        info!("Allies:");
        for a in self.allies.iter() {
            info!("{}", a);
        }
        info!("Enemies:");
        for e in self.enemies.iter() {
            info!("{}", e);
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

    pub fn begin_wave(&mut self) {
        debug!("Wave begin");
        indent!({
            self.allies.iter_mut()
                .chain(self.enemies.iter_mut())
                .for_each(|a| 
                    match a.passives[..] {
                        [ Passive::Resplendence { turn_meter_ratio }, .. ] => {
                            debug!("{} has Resplendence", a);
                            a.set_turn_meter(TURN_METER_THRESHOLD * turn_meter_ratio);
                        },
                        _ => {}
                    }
                );
        })
    }

    pub fn run(& mut self) -> Result {
        self.begin_wave();
        loop {
            self.log_info();
            self.progress_turn_meter();
            
            if let Some(ir) = self.find_actor_index() {
                self.act(ir);
                self.turns += 1;
            }
            else {
                log::debug!("Nobody acts");
            }

            // game over
            let win = self.enemies.iter().all(|e| !e.is_alive());
            let loss = self.allies.iter().all(|a| !a.is_alive());
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
                    self.print_allies();
                    self.print_enemies();
                    panic!("Inconsistent result win: {}, loss: {}, stall: {}", win,loss,stall);
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
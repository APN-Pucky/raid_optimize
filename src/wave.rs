use rand::Rng;

use crate::hero::Hero;
use crate::hero::instance::Instance;

pub struct InstanceRef {
    team: bool,
    index: usize,
}

pub struct Wave {
    allies: Vec<Instance>, // should this be position dependent?
    enemies: Vec<Instance>,
    turns: u32,
    turn_limit: u32,
    initiative_threshold: u32,
}

impl Wave {
    pub fn new(allies: & Vec<&Hero>, enemies : & Vec<&Hero>) -> Wave {
        let mut id = 0;
        let allies = allies.iter().map(|h| {
            id += 1;
            Instance::new(h, id)
        }).collect();
        let enemies = enemies.iter().map(|h| {
            id += 1;
            Instance::new(h, id)
        }).collect();
        Wave {
            allies,
            enemies,
            turns: 0,
            turn_limit: 300,
            initiative_threshold: 1000,
        }
    }

    pub fn get_instance(&self, actor : InstanceRef) -> &Instance {
        if actor.team {
            &self.allies[actor.index]
        }
        else {
            &self.enemies[actor.index]
        }
    }

    pub fn find_actor_index(&self) -> Option<InstanceRef> {
        let mut index = 0;
        let mut found = false;
        let mut max_initiative = 0;
        let mut max_index = 0;
        for actor in self.allies.iter() {
            if actor.get_initiative() > max_initiative {
                max_initiative = actor.get_initiative();
                max_index = index;
                found = true;
            }
            index += 1;
        }
        index = 0;
        for actor in self.enemies.iter() {
            if actor.get_initiative() > max_initiative {
                max_initiative = actor.get_initiative();
                max_index = index;
                found = false;
            }
            index += 1;
        }
        if max_initiative > self.initiative_threshold {
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
        let mut team = self.get_enemy_team(actor);
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
        self.allies.iter_mut().for_each(|a| a.increase_initiative());
        self.enemies.iter_mut().for_each(|a| a.increase_initiative());
    }

    pub fn act(&mut self, actor : InstanceRef) {
        match self.choose_target(&actor) {
            Some(target) => {
                if actor.team {
                    let attacker = &mut self.allies[actor.index];
                    let defender = &mut self.enemies[target.index];
                    attacker.attack(defender);
                    attacker.reset_initiative();
                }
                else {
                    let attacker = &mut self.enemies[actor.index];
                    let defender = &mut self.allies[target.index];
                    attacker.attack(defender);
                    attacker.reset_initiative();
                }
            },
            None => {},
        }
    }

    pub fn game_over(&self) -> bool {
        let mut allies_alive = false;
        let mut enemies_alive = false;
        for ally in self.allies.iter() {
            if ally.is_alive() {
                allies_alive = true;
            }
        }
        for enemy in self.enemies.iter() {
            if enemy.is_alive() {
                enemies_alive = true;
            }
        }
        if !allies_alive {
            log::debug!("Allies dead");
        }
        if !enemies_alive {
            log::debug!("Enemies dead");
        }
        if self.turns >= self.turn_limit {
            log::debug!("Turn limit reached");
        }
        !allies_alive || !enemies_alive || (self.turns >= self.turn_limit)
    }

    pub fn simulate(& mut self) {
        while !self.game_over() {
            self.increase_initiatives();
            match self.find_actor_index() {
                Some(ir) => {
                    self.act(ir);
                    self.turns += 1;
                },
                None => {},
            }
            
        }
    }
}
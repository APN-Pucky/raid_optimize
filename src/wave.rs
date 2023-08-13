use rand::Rng;

use crate::hero::Hero;
use crate::hero::instance::Instance;
use crate::hero::statistics::Statistics;

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

pub struct Result {
    pub win: bool,
    pub loss: bool,
    pub stall : bool,
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

    pub fn get_statistics(&self) -> Vec<&Statistics> {
        self.allies.iter().chain(self.enemies.iter()).map(|i| i.get_statistics()).collect()
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

    pub fn run(& mut self) -> Result {
        loop {
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
                }
            }
        }
        
    }
}
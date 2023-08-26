use crate::{wave::stat::Stat, debug, indent, hero::{instance::Instance, effect::Effect}, info};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {

    pub fn fmt(&self,actor:InstanceIndex) -> String {
        format!("{}-{} [health: {}, turn_meter: {}]", self.heroes[actor].name, actor,self.health[actor], self.turn_meter[actor])
    }

    pub fn log_info(&self) {
        info!("Turn: {}", self.turns); 
        for p in self.players.iter() {
            info!("{}", p.get_name());
            for a in self.get_ally_indices(p.get_team()) {
                info!("{}", a);
            }
        }
    }

    pub fn print_all(&self) {
        for p in self.players.iter() {
            println!("{}", p.get_name());
            for a in self.get_ally_indices(p.get_team()) {
                println!("{}", a);
            }
        }
    }

    pub fn print_allies(&self, ii:InstanceIndex) {
        println!("Allies:");
        for a in self.get_ally_indices(ii) {
            println!("{}", a);
        }
    }

    pub fn print_enemies(&self,ii:InstanceIndex) {
        println!("Enemies:");
        for e in self.get_enemies_indices(ii) {
            println!("{}", e);
        }
    }

}
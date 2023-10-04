use crate::{indent, info};

use super::{Wave, InstanceIndex};

impl Wave<'_> {

    pub fn fmt(&self,actor:InstanceIndex) -> String {
        format!("{}-{} [health: {}, turn_meter: {}, shield: {} = {}]", 
            self.heroes[actor].name, 
            actor,
            self.health[actor], 
            self.turn_meter[actor],
            self.get_shield(actor),
            self.fmt_shield(actor)
        )
    }

    pub fn fmt_shield(&self,actor:InstanceIndex) -> String {
        self.shields[actor].iter().fold(String::new(), |acc, (v,t,_)| format!("{} + {}({}),", acc, v, t))
    }

    pub fn name(&self,actor:InstanceIndex) -> String {
        format!("{}-{}", self.heroes[actor].name, actor)
    }

    pub fn log_info(&self) {
        info!("Turn: {}", self.turns); 
        indent!({
            for p in self.players.iter() {
                info!("{}", p.get_name());
                indent!({
                    for a in self.get_team_indices(p.get_team()) {
                        info!("{}", self.fmt(a));
                    }
                })
            }
        })
    }

    pub fn print_all(&self) {
        for p in self.players.iter() {
            println!("{}", p.get_name());
            for a in self.get_team_indices(p.get_team()) {
                println!("{}", self.fmt(a));
            }
        }
    }

    pub fn print_allies(&self, ii:InstanceIndex) {
        println!("Allies:");
        for a in self.get_ally_indices(ii) {
            println!("{}", self.fmt(a));
        }
    }

    pub fn print_enemies(&self,ii:InstanceIndex) {
        println!("Enemies:");
        for e in self.get_enemies_indices(ii) {
            println!("{}", self.fmt(e));
        }
    }

}
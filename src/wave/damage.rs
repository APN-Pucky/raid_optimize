use crate::{debug, wave::stat::Stat, indent};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn attack_enemy_team(&mut self, actor : InstanceIndex, damage : f32) {
        for a in self.get_enemies_indices(actor){
                self.damage(actor,a,damage)
        }
    }
    pub fn attack_single(&mut self, actor : InstanceIndex, target : InstanceIndex, damage : f32) {
        self.damage(actor,target,damage)
    }

    pub fn damage_all_instances(&mut self,actor: InstanceIndex ,damage : f32) {
        debug!("all instances take {} damage from {}", damage, self.name(actor));
        indent!({
            for i in 0..LEN {
                self.damage(actor,i,damage);
            }
        })
    }

    pub fn damage_hp_burning(&mut self,actor : InstanceIndex,target:InstanceIndex, dmg: f32) {
        debug!("{} takes {} damage from hp_burning from {}", self.name(target), dmg,self.name(actor));
        //TODO track stat
        self.damage(actor,target,dmg);
    }

    pub fn damage_bleed(&mut self,actor : InstanceIndex,target:InstanceIndex, bleed_dmg: f32) {
        debug!("{} takes {} damage from bleed from {}", self.name(target), bleed_dmg,self.name(actor));
        //TODO track stat
        self.damage(actor,target,bleed_dmg);
    }

    pub fn loose_health(&mut self, actor:InstanceIndex, damage: f32) {
        if self.health[actor] < damage {
            self.add_stat(actor,Stat::HealthLost, self.health[actor]);
            self.health[actor] = 0.0;
        }
        else {
            self.add_stat(actor,Stat::HealthLost, damage);
            self.health[actor] -= damage;
        }
        debug!("{} looses {} health to {}",self.name(actor), damage, self.health[actor]);
    }

    pub fn damage(&mut self, actor:InstanceIndex, target:InstanceIndex,damage: f32) {
        debug!("{} takes {} damage from {}", self.name(target), damage,self.name(actor));
        indent!({
            self.add_stat(actor,Stat::DamageTaken, damage);
            self.add_stat(target,Stat::DamageDone, damage);
            let dmg = self.shield_loose(actor,damage);
            self.loose_health(actor,dmg);
        })
    }

}
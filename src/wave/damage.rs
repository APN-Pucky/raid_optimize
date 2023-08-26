use rand::Rng;

use crate::{debug, wave::stat::Stat, indent, hero::faction::Faction};

use super::{Wave, InstanceIndex};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn attack_enemy_team(&mut self, actor : InstanceIndex, damage : f32) {
        for a in self.get_enemies_indices(actor){
            self.attack_single(actor,a,damage)
        }
    }
    pub fn attack_single(&mut self, actor : InstanceIndex, target : InstanceIndex, damage : f32) {
        debug!("{} attacks {} with {} attack", self.name(actor), self.name(target),damage);
        indent!({
            self.add_stat(actor,Stat::Attacks, 1.0);
            self.add_stat(target,Stat::Defends, 1.0);
            let mut rng = rand::thread_rng();
            let crit = rng.gen::<f32>() < self.get_crit_rate(actor);
            let mut attack = damage;
            indent!({
            if crit {
                self.add_stat(actor,Stat::CriticalStrikes, 1.0);
                self.add_stat(target,Stat::CriticalStriked, 1.0);
                let crit = self.get_crit_damage(actor);
                let mut tenacity = self.get_tenacity(target);
                if tenacity > crit {
                    tenacity = crit;
                }
                let crit_rate = crit - tenacity;
                self.add_stat(actor, Stat::CriticalDamage, attack  * crit_rate  );
                self.add_stat(target, Stat::CriticalDamaged, attack  * crit_rate  );
                self.add_stat(target,Stat::SavedByTenacity, attack  * tenacity );
                self.add_stat(actor,Stat::LostToTenacity, attack  * tenacity );
                attack = (attack * crit_rate);
                debug!("{} critical attacks {} ({}%={}%-{}%)", self.name(actor),self.name(target),crit_rate*100.,crit*100.,tenacity*100.);
            }
            });
            self.add_stat(actor,Stat::Attack, attack);
            self.add_stat(target,Stat::Attacked, attack);
            let mut def = self.get_defense(target);
    
            let p = self.get_piercing(actor);
            let pierce = (def  * p); // TODO handle rounding
            self.add_stat(actor,Stat::Piercing, pierce);
            self.add_stat(target,Stat::Pierced, pierce);
            debug!("{} pierces {} defense of {} ({}%)", self.name(actor), pierce, def, p*100.);
            def -= pierce;
             
            if attack -def > 0. {
                self.damage(actor,target, (attack - def),true,true);
            }
            else {
                self.add_stat(actor,Stat::Blocked, attack);
                self.add_stat(target,Stat::BlockedBy, attack);
                debug!("{} blocked by {} defense of {}", attack, def, self.name(target));
            }
        })
    }

    pub fn damage_hp_burning(&mut self,actor : InstanceIndex,target:InstanceIndex, dmg: f32) {
        debug!("{} takes {} damage from hp_burning from {}", self.name(target), dmg,self.name(actor));
        //TODO track stat
        self.damage(actor,target,dmg,false,false);
    }

    pub fn damage_bleed(&mut self,actor : InstanceIndex,target:InstanceIndex, bleed_dmg: f32) {
        debug!("{} takes {} damage from bleed from {}", self.name(target), bleed_dmg,self.name(actor));
        //TODO track stat
        self.damage(actor,target,bleed_dmg,false,false);
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

    pub fn damage(&mut self, actor:InstanceIndex, target:InstanceIndex,damage: f32, reflect:bool,leech: bool) {
        debug!("{} takes {} damage from {}", self.name(target), damage,self.name(actor));
        indent!({
            let mut damage = damage;
            if self.heroes[target].faction == Faction::DoomLegion {
                let n = self.count_self_buffs(target).min(5) as f32;
                let xfact = self.team_bonds[self.teams[target]][Faction::DoomLegion];
                let r = 1.0 - xfact*n;
                damage = damage *r;
                debug!("{} has {}*{} DoomLegion buffs -> damage * {}", self.name(target), n,xfact, r);
            }

            self.add_stat(actor,Stat::DamageTaken, damage);
            self.add_stat(target,Stat::DamageDone, damage);
            let dmg = self.shield_loose(actor,damage);
            self.loose_health(actor,dmg);
            if leech {
                self.leech(actor,target,dmg);
            }
            if reflect {
                self.reflect_damage(target,actor,dmg * self.get_damage_reflect(target));
            }
        })
    }

    pub fn leech(&mut self, actor:InstanceIndex, target:InstanceIndex,dmg: f32) {
        let leech = dmg * self.get_leech(actor);
        if leech > 0.0 {
            debug!("{} leeches {} health from {}", self.name(actor), leech,self.name(target));
            indent!({
                self.add_stat(actor,Stat::Leeched, leech);
                self.add_stat(target,Stat::LeechedOf, leech);
                self.heal(actor,leech);
            })
        }
    }

    pub fn reflect_damage(&mut self, actor:InstanceIndex, target:InstanceIndex,damage: f32) {
        debug!("{} reflects {} damage to {}", self.name(actor), damage,self.name(target));
        indent!({
            self.add_stat(actor,Stat::DamageReflected, damage);
            self.add_stat(target,Stat::DamageReflecteded, damage);
            self.damage(actor,target,damage,false,false);
        })
    }

}
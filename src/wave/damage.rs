use enum_map::EnumMap;
use rand::Rng;

use crate::{debug, wave::{stat::Stat, heroes::marville::fish_guardian::FishGuardian}, indent, data::{faction::Faction, mark::Mark, skill::{Skill, is_basic_attack,  }, effect::Effect}};

use super::{Wave, InstanceIndex, heroes::{hazier::bloodlust_strike::BloodlustStrike, BasicAttack}};

impl Wave<'_> {
    pub fn attack_enemy_team(&mut self, actor : InstanceIndex, damage : f32,skill:&Skill) {
        for a in self.get_enemies_indices(actor){
            self.attack_single(actor,a,damage,skill)
        }
    }
    pub fn attack_single(&mut self, actor : InstanceIndex, target : InstanceIndex, d: f32,skill:&Skill) {
        debug!("{} attacks {} to {}", self.name(actor),d, self.fmt(target));
        let mut damage = d;
        indent!({
            if is_basic_attack(skill){
                damage *= self.get_basic_attack_damage_ratio(actor);
            } else {
                damage *= self.get_skill_damage_ratio(actor);
            }
            self.add_stat(actor,Stat::Attacks, 1.0);
            self.add_stat(target,Stat::Defends, 1.0);
            let mut rng = rand::thread_rng();
            let crit = rng.gen::<f32>() < self.get_crit_rate(actor);
            let mut attack = damage;
            let mut p = self.get_piercing(actor,skill);
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
                    attack = attack * crit_rate;
                    debug!("{} critical attacks {} ({}%={}%-{}%)", self.name(actor),self.name(target),crit_rate*100.,crit*100.,tenacity*100.);
                    if self.get_faction(actor) == Faction::NamelessBrotherhood {
                        if rng.gen::<f32>() < 0.5 {
                            p += self.get_bond(actor,Faction::NamelessBrotherhood);
                            debug!("{} has {} bond with NamelessBrotherhood -> piercing + {}", self.name(actor), self.get_bond(actor,Faction::NamelessBrotherhood), self.get_bond(actor,Faction::NamelessBrotherhood));
                        }
                    }
                    self.on_critical_strike_tifya(actor,skill);
                }
            });

            self.add_stat(actor,Stat::Attack, attack);
            self.add_stat(target,Stat::Attacked, attack);
            let mut def = self.get_defense(target);
    
            let pierce = def  * p; // TODO handle rounding
            self.add_stat(actor,Stat::Piercing, pierce);
            self.add_stat(target,Stat::Pierced, pierce);
            debug!("{} pierces {} defense of {} ({}%)", self.name(actor), pierce, def, p*100.);
            def -= pierce;
             
            if attack -def > 0. {
                self.damage(actor,target, attack - def,skill,true,true,crit);
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
        self.damage(actor,target,dmg,&Skill::None,false,false,false);
    }

    pub fn damage_bleed(&mut self,actor : InstanceIndex,target:InstanceIndex, bleed_dmg: f32) {
        debug!("{} takes {} damage from bleed from {}", self.name(target), bleed_dmg,self.name(actor));
        //TODO track stat
        self.damage(actor,target,bleed_dmg,&Skill::None,false,false,false);
    }

    pub fn loose_health(&mut self, actor:InstanceIndex, damage: f32) {
        if self.health[actor] < damage {
            self.add_stat(actor,Stat::HealthLost, self.health[actor]);
            self.health[actor] = 0.0;
            self.on_fatal_damage_maya(actor);
        }
        else {
            self.add_stat(actor,Stat::HealthLost, damage);
            self.health[actor] -= damage;
        }
        debug!("{} looses {} health to {}",self.name(actor), damage, self.health[actor]);
    }

    pub fn damage(&mut self, actor:InstanceIndex, target:InstanceIndex,damage: f32, skill : &Skill, reflect:bool,leech: bool,crit:bool) {
        debug!("{} takes {} damage from {}", self.name(target), damage,self.name(actor));
        indent!({
            if self.has_effect(target, Effect::DamageImmunity) {
                debug!("{} has DamageImmunity -> damage * 0", self.name(target));
                return;
            }
            let mut damage = damage;
            if self.has_effect(target, Effect::ConsolidationI) {
                    let xfact = 0.80;
                    damage = damage * xfact;
                    debug!("{} has ConsolidationII -> damage * {}", self.name(target), xfact);
            }
            if self.has_effect(target, Effect::ConsolidationII) {
                    let xfact = 0.60;
                    damage = damage * xfact;
                    debug!("{} has ConsolidationII -> damage * {}", self.name(target), xfact);
            }
            if self.has_effect(target, Effect::FishShoal) {
                    // find hero with FishGuardian skill
                    let mut red = 0.0;
                    for i in self.get_ally_indices(target) {
                        match self.heroes[i].skills[..] {
                            [ Skill::FishGuardian(FishGuardian {max_hp_restore_ratio , damage_reduction,.. }),..] => {
                                red = damage_reduction;
                                self.heal(target,i,self.get_max_health(target)* max_hp_restore_ratio);
                                self.effects[i].remove_layer(Effect::FishShoal);
                                break;
                            },
                            _ => {}
                        }
                    }
                    let xfact = 1.0-red;
                    damage = damage * xfact;
                    debug!("{} has FishShoal -> damage * {}", self.name(target), xfact);
            }
            if self.get_faction(target) == Faction::DoomLegion {
                let n = self.bonds_counter[target] as f32;
                let xfact = self.get_bond(target,Faction::DoomLegion);
                let r = 1.0 - xfact*n;
                damage = damage *r;
                debug!("{} has {}*{} DoomLegion buffs -> damage * {}", self.name(target), n,xfact, r);
            }
            if self.get_faction(actor) == Faction::EternalSect {
                if self.has_debuff(target) {
                    let xfact = self.get_bond(actor,Faction::EternalSect);
                    damage = damage * xfact;
                    debug!("{} has {} bond with EternalSect and {} has debuff -> damage * {}", self.name(actor), xfact, self.name(target), xfact);
                }
            }
            if self.get_faction(actor) == Faction::SwordHarborGuards {
                if self.health[actor] > 0.5 * self.get_max_health(actor) {
                    let xfact = 1.0 + self.get_bond(actor,Faction::SwordHarborGuards);
                    damage = damage * xfact;
                    debug!("{} has {} bond with SwordHarborGuards and health > 50% -> damage * {}", self.name(actor), xfact, xfact);
                }
            }
            if self.get_faction(target) == Faction::SwordHarborGuards {
                if self.health[target] < 0.5 * self.get_max_health(target) {
                    let xfact = 1.0 - self.get_bond(target,Faction::SwordHarborGuards);
                    damage = damage * xfact;
                    debug!("{} has {} bond with SwordHarborGuards and health < 50% -> damage * {}", self.name(target), xfact, xfact);
                }
            }
            for p in &self.heroes[actor].skills {
                match p {
                    Skill::BloodlustStrike (BloodlustStrike{damage_reduction_buffs,damage_reduction_nobuffs,..}) => {
                        if self.has_buff(target) {
                            damage = damage * ( 1.0 - damage_reduction_buffs);
                        }
                        else {
                            damage = damage * ( 1.0 - damage_reduction_nobuffs);
                        }
                    }
                    _ => {}
                }
            }

            let mut mat :EnumMap<Mark,EnumMap<Mark,f32>> =   EnumMap::default(); 

            mat[Mark::Red][Mark::Red]   = 1.00;
            mat[Mark::Red][Mark::Blue]  = 1.25;
            mat[Mark::Red][Mark::Green] = 0.75;
            mat[Mark::Red][Mark::Force] = 1.00;

            mat[Mark::Blue][Mark::Red]   = 0.75;
            mat[Mark::Blue][Mark::Blue]  = 1.00;
            mat[Mark::Blue][Mark::Green] = 1.25;
            mat[Mark::Blue][Mark::Force] = 1.00;

            mat[Mark::Green][Mark::Red]   = 1.25;
            mat[Mark::Green][Mark::Blue]  = 0.75;
            mat[Mark::Green][Mark::Green] = 1.00;
            mat[Mark::Green][Mark::Force] = 1.00;

            mat[Mark::Force][Mark::Red]   = 1.00;
            mat[Mark::Force][Mark::Blue]  = 1.00;
            mat[Mark::Force][Mark::Green] = 1.00;
            mat[Mark::Force][Mark::Force] = 1.00;

            let fact = mat[self.get_mark(actor)][self.get_mark(target)];
            damage = damage * fact;
            if fact != 1.0 {
                debug!("{} has {} mark against {} -> damage * {}", self.name(actor), self.get_mark(actor), self.get_mark(target), fact);
            }
            

            self.add_stat(actor,Stat::DamageTaken, damage);
            self.add_stat(target,Stat::DamageDone, damage);
            let dmg = self.shield_loose(actor,target,damage);
            self.loose_health(target,dmg);
            self.on_damage_dealt(actor,target,dmg,skill,reflect,leech,crit);
            
        })
    }

    pub fn on_damage_dealt(&mut self, actor:InstanceIndex, target:InstanceIndex,dmg: f32,skill:&Skill,reflect : bool, leech : bool, crit:bool) {
        self.on_damage_dealt_alahan(actor,target,dmg,skill);
        self.on_damage_dealt_maya(actor,target,dmg,skill);
        if leech {
            self.leech(actor,target,dmg,crit);
        }
        if reflect {
            self.reflect_damage(target,actor,dmg * self.get_damage_reflect(target));
        }
        if self.has_effect(target,Effect::CounterAttack) {
            self.attack_single(target,actor,self.get_attack_damage(target), &Skill::BasicAttack(BasicAttack::default()));
        }
    }

    pub fn leech(&mut self, actor:InstanceIndex, target:InstanceIndex,dmg: f32,crit:bool) {
        let mut fleech = self.get_leech(actor);
        if crit {
            for p in &self.heroes[actor].skills {
                match p {
                    Skill::BloodlustStrike (BloodlustStrike{leech,..}) => {
                        fleech += leech;
                    }
                    _ => {}
                }
            }
        }
        let leech = dmg * fleech;
        if leech > 0.0 {
            debug!("{} leeches {} health from {}", self.name(actor), leech,self.name(target));
            indent!({
                self.add_stat(actor,Stat::Leeched, leech);
                self.add_stat(target,Stat::LeechedOf, leech);
                self.heal(actor,actor,leech);
            })
        }
    }

    pub fn reflect_damage(&mut self, actor:InstanceIndex, target:InstanceIndex,damage: f32) {
        if damage > 0. {
            debug!("{} reflects {} damage to {}", self.name(actor), damage,self.name(target));
            indent!({
                self.add_stat(actor,Stat::DamageReflected, damage);
                self.add_stat(target,Stat::DamageReflecteded, damage);
                self.damage(actor,target,damage,&Skill::None,false,false,false);
            })
        }
        if damage < 0. {
            panic!("{} reflects negative damage {} to {}", self.name(actor), damage,self.name(target));
        }
    }

}
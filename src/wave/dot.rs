use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}}};

use super::{InstanceIndex, Wave};


impl<const LEN:usize> Wave<'_,LEN> {

    pub fn dot_heal(&mut self, actor : InstanceIndex) {
        // apply heal
        let n = self.effects[actor].get(Effect::Heal);
        if n> 0 {
            let b : &Vec<(u32,InstanceIndex)> = &self.effects[actor].em[Effect::Heal];
            let nn: InstanceIndex = b.last().unwrap().1;
            let heal = self.get_max_health(actor) * 0.05 * n as f32;
            self.heal(nn,actor,heal);
        }
    }

    pub fn dot_bleed(&mut self, actor : InstanceIndex) {
        // apply bleed
        let n = self.effects[actor].get(Effect::Bleed).min(10);
        if n > 0 {
            let b : &Vec<(u32,InstanceIndex)> = &self.effects[actor].em[Effect::Bleed];
            // get inflictor
            let inflictor = b.last().unwrap().1;
            //let nn: u32= b.iter().map(|(n,_)| n).sum();
            let dmg_vec = vec![0.30,0.50,0.70,0.90,1.05,1.20,1.35,1.45,1.55,1.65];
            let bleed_dmg = self.get_attack_damage(inflictor) * dmg_vec[n as usize -1] ;
            let mastery = self.get_mastery(inflictor);
            self.damage_bleed(inflictor,actor,bleed_dmg * (1.0 + mastery));
        }
    }

    pub fn dot_hp_burning(&mut self, actor : InstanceIndex) {
        // apply HP burning
        let n = self.effects[actor].get(Effect::HPBurning);
        if n > 0 {
            let b : &Vec<(u32,InstanceIndex)> = &self.effects[actor].em[Effect::HPBurning];
            // get inflictor
            let inflictor : InstanceIndex= b.last().unwrap().1;
            let mut hp_burn_dmg : f32 = self.get_max_health(actor) * 0.08 * n as f32;
            let mastery = self.get_mastery(inflictor);
            hp_burn_dmg *= 1.0 + mastery;
            let max = 0.3*self.get_max_health(inflictor);
            if hp_burn_dmg > max {
                debug!("{} HP burning damage capped at {}", self.name(actor), max);
                hp_burn_dmg = max;
            }
            self.damage_hp_burning(inflictor,actor,hp_burn_dmg);
        }
    }
}
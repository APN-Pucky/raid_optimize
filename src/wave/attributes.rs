use crate::{debug, indent, hero::{effect::Effect, skill::{Skill, get_targets, execute_skill}, Hero, instance::Instance}};

use super::{InstanceIndex, Wave};


impl<const LEN:usize> Wave<'_,LEN> {

    pub fn get_healing_effect(&self, actor: InstanceIndex) -> f32 {
        // TODO handle healing buff/debuff
        self.get_hero(actor).healing_effect
    }


    pub fn is_alive(&self, actor : InstanceIndex) -> bool {
        self.health[actor] > 0.0
    }
    pub fn is_dead(&self, actor : InstanceIndex) -> bool {
        !self.is_alive(actor)
    }

    #[inline]
    pub fn get_hero(&self, actor : InstanceIndex) -> &Hero {
        &self.heroes[actor]
    }

    pub fn get_max_health(&self, actor : InstanceIndex) -> f32 {
        self.get_hero(actor).health
    }

    pub fn get_speed(&self, actor : InstanceIndex) -> f32 {
        let mut fact = 1.0;
        if self.effects[actor].has(Effect::SpeedUpI) {
            fact *= 1.2;
        }
        if self.effects[actor].has(Effect::SpeedDownII) {
            fact *= 0.6;
        }
        self.get_hero(actor).speed  * fact 
    }

    pub fn get_effect_resistance(&self,actor : InstanceIndex) -> f32 {
        // TODO handle effect resistance buff/debuff
        let mut fact = 1.0;
        if self.effects[actor].has(Effect::EffectResistanceDownII) {
            fact = 0.5;
        }
        self.get_hero(actor).effect_resistance * fact
    }

    pub fn get_basic_attack_damage(&self,actor : InstanceIndex) -> f32 {
        if self.has_effect(actor,Effect::RippleII) {
            self.get_attack(actor) * 1.40
        }
        else {
            self.get_attack(actor)
        }
    }

    pub fn get_attack(&self,actor : InstanceIndex) -> f32 {
        // TODO handle attack buff/debuff
        if self.has_effect(actor,Effect::AttackUpII) {
            self.get_hero(actor).attack  * 1.4
        }
        else {
            self.get_hero(actor).attack 
        }
    }
    
    pub fn get_attack_damage(&self,actor : InstanceIndex) -> f32 {
        self.get_attack(actor) 
    }

    pub fn get_defense(&self,actor:InstanceIndex) -> f32 {
        // TODO handle defense buff/debuff
        self.get_hero(actor).defense
    }

}
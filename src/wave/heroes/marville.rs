use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::{Effect, is_attribute_debuff, is_dot}}, roll};


impl Wave<'_> {
    pub fn execute_skill_marville(&mut self,  skill : &Skill, actor :InstanceIndex, _target :InstanceIndex, ) {
        match skill.data {
            SkillData::FishWaterball {  attack_damage_ratio, act_chance, ..} =>{
                let damage = self.get_attack_damage(actor) * attack_damage_ratio;
                self.attack_enemy_team(actor, damage ,skill);
                self.refresh_enemy_team(actor, &is_attribute_debuff);
                if roll(act_chance){
                    // take another turn
                    self.act(actor);
                }
            },
            SkillData::CleanOcean{ restore_max_hp_ratio, cleanse_dot_layers, consolidation_turns, block_removal_turns }=> {
                self.restore_max_hp_ratio_own_team(actor, restore_max_hp_ratio);
                self.cleanse_team(actor, &is_dot, cleanse_dot_layers);
                self.cleanse_team(actor, &is_attribute_debuff, 999);
                self.inflict_ally_team(actor, Effect::BlockRemoval, 1.0, block_removal_turns);
                self.inflict_ally_team(actor, Effect::ConsolidationII, 1.0, consolidation_turns);

            },
            SkillData::FishDive{ restore_fish_shoal } => {
                self.restore_to_highest_ally_health_percentage(actor);
                for i in self.get_ally_indices(actor) {
                    if self.has_effect(i, Effect::FishShoal) {
                        self.reduce_cooldowns(i);
                    }
                }
                for _ in 0..restore_fish_shoal {
                    self.inflict_ally_team(actor, Effect::FishShoal, 1.0, 999)
                }
            }
            _ => {}
        }
    }

    pub fn on_begin_wave_marville(&mut self) {
        for i in self.get_indices() {
            match self.heroes[i].skills[..] {
                [Skill {data : SkillData::FishGuardian {..},..},..] => {
                    self.inflict_ally_team(i, Effect::FishShoal, 1.0, 999);
                    self.inflict_ally_team(i, Effect::FishShoal, 1.0, 999);
                },
                _ => {}
            }
        }
    }

    pub fn on_turn_start_marville(&mut self, actor :InstanceIndex) {
       for i in self.get_ally_indices(actor) {
            match self.heroes[i].skills[..] {
                [Skill {data : SkillData::FishGuardian {restore_fish_shoal ,..},..},..] => {
                    for i in 0 .. restore_fish_shoal {
                        self.inflict_single(i as InstanceIndex, actor, Effect::FishShoal, 1.0, 999)
                    }
                },
                _ => {}
            }
       } 
    }
}
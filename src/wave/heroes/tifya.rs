use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::{Effect, is_debuff}, passive::Passive }};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn execute_skill_tifya(&mut self,  skill : &Skill, actor :InstanceIndex, target :InstanceIndex, ) {
        //let wave = self;
        let attacker = actor;
        let defender = target;
        match skill {
            Skill::Generic{ basic_attack,cooldown, subskills ,..} => {
                for ss in subskills {
                    self.execute_subskill(ss, actor, target,skill);
                }
            },
            Skill::ScarletSlash { basic_attack, attack_damage_ratio, .. } => {
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            },
            Skill::LeavesStorm { basic_attack, attack_damage_ratio, .. } => {
                self.attack_enemy_team(attacker,   self.get_attack_damage(attacker)  *attack_damage_ratio * (1. + 0.02 *self.effects[actor].get(Effect::ScarletSakura) as f32) , skill);
            },
            Skill::ScaletMultiStrike{ basic_attack, attack_damage_ratio, .. } => {
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
                self.attack_single(attacker, defender,  self.get_attack_damage(attacker)  *attack_damage_ratio, skill);
            },
            _ => {}
        }
    }

    pub fn on_critical_strike_tifya(&mut self, actor:InstanceIndex, skill :&Skill) {
        match skill {
            Skill::ScarletSlash{..} => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            Skill::ScaletMultiStrike {.. } => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            _ => {}
        }
    }

    pub fn after_action_tifya(&mut self, actor:InstanceIndex) {
            if !self.team_acted[self.teams[actor]] {
                for i in 0..LEN {
                    if self.teams[i] == self.teams[actor] {
                        if i != actor {
                            match self.heroes[i].passives[..] {
                                [Passive::SharpInstinct,..] => {
                                    for s in &self.heroes[i].skills {
                                        match s {
                                            Skill::LeavesStorm { .. } => {
                                                self.execute_skill(&s,i,i)
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        else {
                            match self.heroes[actor].passives[..] {
                                [Passive::SharpInstinct,..] => {
                                    for s in &self.heroes[actor].skills {
                                        match s {
                                            Skill::LeavesStorm { .. } => {
                                                self.inflict_buff_single(actor, actor, Effect::Stealth,1);
                                                self.execute_skill(&s,i,i)
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    if self.teams[i] != self.teams[actor] {
                        match self.heroes[i].passives[..] {
                            [Passive::SharpInstinct,..] => {
                                self.cleanse(i, &is_debuff, 999);
                                self.act(i);
                            }
                            _ => {}
                        }
                    }
                }
            }
    }
}
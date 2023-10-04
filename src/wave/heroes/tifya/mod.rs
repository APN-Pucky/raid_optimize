use crate::{wave::{Wave, InstanceIndex}, data::{skill::{Skill, SkillData}, effect::{Effect, is_debuff} }};

pub mod scarlet_multi_strike;
pub mod leaves_storm;
pub mod scarlet_slash;

impl Wave<'_> {
    pub fn on_critical_strike_tifya(&mut self, actor:InstanceIndex, skill :&Skill) {
        match skill.data {
            SkillData::ScarletSlash{..} => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            SkillData::ScaletMultiStrike {.. } => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            _ => {}
        }
    }

    pub fn after_action_tifya(&mut self, actor:InstanceIndex) {
            if !self.team_acted[self.teams[actor]] {
                for i in 0..self.len(){
                    if self.teams[i] == self.teams[actor] {
                        if i != actor {
                            match self.heroes[i].skills[..] {
                                [Skill { data : SkillData::SharpInstinct,..},..] => {
                                    for s in &self.heroes[i].skills {
                                        match s.data {
                                            SkillData::LeavesStorm { .. } => {
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
                            match self.heroes[actor].skills[..] {
                                [Skill { data : SkillData::SharpInstinct,..},..] => {
                                    for s in &self.heroes[actor].skills {
                                        match s.data {
                                            SkillData::LeavesStorm { .. } => {
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
                        match self.heroes[i].skills[..] {
                            [Skill { data : SkillData::SharpInstinct,..},..] => {
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
#[cfg(test)]
mod tests;
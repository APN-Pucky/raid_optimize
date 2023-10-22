use crate::{
    data::{effect::Effect, skill::Skill},
    wave::{has_skill, InstanceIndex, Wave},
};

use self::{scarlet_multi_strike::ScarletMultiStrike, scarlet_slash::ScarletSlash};

pub mod leaves_storm;
pub mod scarlet_multi_strike;
pub mod scarlet_slash;

impl Wave<'_> {
    pub fn on_critical_strike_tifya(&mut self, actor: InstanceIndex, skill: &Skill) {
        match skill {
            Skill::ScarletSlash(ScarletSlash { .. }) => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            Skill::ScarletMultiStrike(ScarletMultiStrike { .. }) => {
                self.inflict_buff_single(actor, actor, Effect::ScarletSakura, 999)
            }
            _ => {}
        }
    }

    pub fn after_action_tifya(&mut self, actor: InstanceIndex) {
        if !self.team_acted[self.teams[actor]] {
            for i in self.get_indices_iter() {
                if self.teams[i] == self.teams[actor] {
                    if i != actor {
                        if has_skill!(self, i, Skill::SharpInstinct) {
                            for s in &self.heroes[i].skills {
                                if let Skill::LeavesStorm { .. } = s {
                                    self.execute_skill(&s, i, i)
                                }
                            }
                        }
                    } else {
                        if has_skill!(self, actor, Skill::SharpInstinct) {
                            for s in &self.heroes[actor].skills {
                                if let Skill::LeavesStorm { .. } = s {
                                    self.inflict_buff_single(actor, actor, Effect::Stealth, 1);
                                    self.execute_skill(&s, i, i)
                                }
                            }
                        }
                    }
                }
                if self.teams[i] != self.teams[actor] {
                    if has_skill!(self, i, Skill::SharpInstinct) {
                        self.cleanse(i, Effect::is_debuff, 999);
                        self.act(i);
                    }
                }
            }
        }
    }
}

use super::test_hero;
test_hero!(Tifya);

use crate::hero::{passive::Passive, skill::{execute_skill, Skill}, effect::{Effect, is_debuff}};

use super::{InstanceIndex, Wave};

impl<const LEN:usize> Wave<'_,LEN> {
    pub fn after_action_sharp_instinct(&mut self, actor:InstanceIndex) {
            if !self.team_acted[self.teams[actor]] {
                for i in 0..LEN {
                    if self.teams[i] == self.teams[actor] {
                        if i != actor {
                            match self.heroes[i].passives[..] {
                                [Passive::SharpInstinct,..] => {
                                    for s in &self.heroes[i].skills {
                                        match s {
                                            Skill::LeavesStorm { .. } => {
                                                execute_skill(&s,i,i,self)
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
                                                execute_skill(&s,i,i,self)
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
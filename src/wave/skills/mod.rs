use std::cmp;

use crate::{
    data::{
        effect::Effect,
        faction::Faction,
        skill::{get_cooldown, is_basic_attack, is_passive, is_reducable, Generic, Skill},
    },
    debug, indent,
};

use super::{stat::Stat, InstanceIndex, Wave};

pub mod subskills;

pub type SkillIndex = usize;
impl<'a> Wave<'a> {
    pub fn get_ready_skills(&self, actor: InstanceIndex) -> Vec<&'a Skill> {
        self.heroes[actor]
            .skills
            .iter()
            .zip(self.cooldowns[actor].iter())
            .filter_map(|(s, c)| {
                if *c == 0
                    && !is_passive(s)
                    && self.effects[actor].get(Effect::Stun) == 0
                    && self.effects[actor].get(Effect::Freeze) == 0
                    && (self.effects[actor].get(Effect::Silence) == 0 || is_basic_attack(s))
                {
                    Some(s)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Wave<'_> {
    pub fn pre_execute_skill(
        &mut self,
        actor: InstanceIndex,
        _target: InstanceIndex,
        skill: &Skill,
    ) {
        debug!("{} pre_execute_skill {}", self.name(actor), skill);
        indent!({
            if self.get_faction(actor) == Faction::HiddenWave {
                if is_basic_attack(skill) {
                    self.inflict_single(actor, actor, Effect::FactionHiddenWaveAttack, 1.0, 2);
                } else {
                    self.inflict_single(actor, actor, Effect::FactionHiddenWaveSkill, 1.0, 2);
                }
            }
        })
    }

    pub fn get_max_cooldown(&self, actor: InstanceIndex, skill: SkillIndex) -> u32 {
        get_cooldown(&self.get_hero(actor).skills[skill])
    }

    pub fn get_skill_indices_iter(&self, actor: InstanceIndex) -> impl Iterator<Item = SkillIndex> {
        0..self.get_hero(actor).skills.len()
    }
    pub fn get_skill_indices(&self, actor: InstanceIndex) -> Vec<SkillIndex> {
        self.get_skill_indices_iter(actor).collect::<Vec<_>>()
    }

    pub fn increase_all_cooldowns(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        value: u32,
    ) -> u32 {
        let mut r = 0;
        for s in self.get_skill_indices_iter(target) {
            r += self.increase_cooldowns(actor, target, s, value);
        }
        r
    }

    pub fn increase_cooldowns(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        si: SkillIndex,
        value: u32,
    ) -> u32 {
        self.add_stat(actor, Stat::IncreaseSkillCooldown, 1.);
        let s = self.cooldowns[actor][si];
        self.cooldowns[actor][si] = cmp::max(
            self.get_max_cooldown(actor, si),
            self.cooldowns[actor][si] + value,
        );
        self.cooldowns[actor][si] - s
    }

    pub fn cooldown_s(&mut self, actor: InstanceIndex, skill: &Skill) {
        let si = self.get_skill_index(actor, skill);
        self.cooldowns[actor][si] = self.get_max_cooldown(actor, si);
    }

    pub fn get_skill_index(&self, actor: InstanceIndex, skill: &Skill) -> SkillIndex {
        self.get_hero(actor)
            .skills
            .iter()
            .position(|s| s == skill)
            .unwrap()
    }

    pub fn get_skill(&self, actor: InstanceIndex, skill_index: SkillIndex) -> &Skill {
        &self.get_hero(actor).skills[skill_index]
    }

    pub fn reset_skill(
        &mut self,
        actor: InstanceIndex,
        target: InstanceIndex,
        skill_index: SkillIndex,
    ) {
        self.add_stat(actor, Stat::ResetSkill, 1.);
        if is_reducable(self.get_skill(target, skill_index)) {
            self.cooldowns[target][skill_index] = 0;
        }
    }

    pub fn reset_all_skills(&mut self, actor: InstanceIndex, target: InstanceIndex) {
        for si in 0..self.cooldowns[target].len() {
            self.reset_skill(actor, target, si);
        }
    }

    pub fn turn_reduce_cooldowns(&mut self, actor: InstanceIndex) {
        debug!(
            "Reducing cooldowns for {} ({}):",
            self.name(actor),
            self.cooldowns[actor].len()
        );
        self.cooldowns[actor]
            .iter_mut()
            .for_each(|c| *c = c.saturating_sub(1));
        indent!({
            for (i, c) in self.cooldowns[actor].iter().enumerate() {
                debug!("{}: {}", self.get_skill(actor, i), c);
            }
        })
    }

    pub fn reduce_cooldowns(&mut self, actor: InstanceIndex) {
        debug!(
            "Reducing cooldowns for {} ({}):",
            self.name(actor),
            self.cooldowns[actor].len()
        );
        self.cooldowns[actor]
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| is_reducable(&self.heroes[actor].skills[*i as SkillIndex]))
            .for_each(|(_, c)| *c = c.saturating_sub(1));
        indent!({
            for (i, c) in self.cooldowns[actor].iter().enumerate() {
                debug!("{}: {}", self.get_skill(actor, i), c);
            }
        })
    }

    pub fn reset_all_cooldowns(&mut self, actor: InstanceIndex) {
        debug!(
            "Resetting cooldowns for {} ({}):",
            self.name(actor),
            self.cooldowns[actor].len()
        );
        self.cooldowns[actor].iter_mut().for_each(|c| *c = 0);
    }

    pub fn cooldown(&mut self, actor: InstanceIndex, skill: SkillIndex) {
        self.cooldowns[actor][skill] = get_cooldown(&self.get_hero(actor).skills[skill]);
    }

    pub fn is_ready(&mut self, actor: InstanceIndex, skill: SkillIndex) -> bool {
        self.cooldowns[actor][skill] == 0
    }

    pub fn execute_generic_skill(
        &mut self,
        skill: &Skill,
        actor: InstanceIndex,
        target: InstanceIndex,
    ) {
        if let Skill::Generic(Generic { subskills, .. }) = skill {
            for ss in subskills {
                self.execute_subskill(&ss, actor, Some(target), skill);
            }
        }
    }

    //}
}

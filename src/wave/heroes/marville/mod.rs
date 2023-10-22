use crate::{
    data::{effect::Effect, skill::Skill},
    wave::{has_skill, InstanceIndex, Wave},
};

use self::fish_guardian::FishGuardian;

pub mod clean_ocean;
pub mod fish_dive;
pub mod fish_guardian;
pub mod fish_waterball;

impl Wave<'_> {
    pub fn on_begin_wave_marville(&mut self) {
        for i in self.get_indices_iter() {
            if has_skill!(self, i, Skill::FishGuardian(_)) {
                self.inflict_ally_team(i, Effect::FishShoal, 1.0, 999);
                self.inflict_ally_team(i, Effect::FishShoal, 1.0, 999);
            }
        }
    }

    pub fn on_turn_start_marville(&mut self, actor: InstanceIndex) {
        for i in self.get_ally_indices(actor) {
            for p in &self.heroes[i].skills {
                if let Skill::FishGuardian(FishGuardian {
                    restore_fish_shoal, ..
                }) = *p
                {
                    for _j in 0..restore_fish_shoal {
                        self.inflict_single(i, actor, Effect::FishShoal, 1.0, 999)
                    }
                }
            }
        }
    }
}

use super::test_hero;
test_hero!(Marville);

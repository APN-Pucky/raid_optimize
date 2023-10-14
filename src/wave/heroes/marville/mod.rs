use crate::{wave::{Wave, InstanceIndex}, data::{skill::Skill, effect::Effect}};

use self::fish_guardian::FishGuardian;

pub mod fish_dive;
pub mod fish_guardian;
pub mod fish_waterball;
pub mod clean_ocean;

impl Wave<'_> {
    pub fn on_begin_wave_marville(&mut self) {
        for i in self.get_indices_iter() {
            match self.heroes[i].skills[..] {
                [Skill::FishGuardian (FishGuardian{..}),..] => {
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
                [Skill::FishGuardian (FishGuardian{restore_fish_shoal ,..}),..] => {
                    for i in 0 .. restore_fish_shoal {
                        self.inflict_single(i as InstanceIndex, actor, Effect::FishShoal, 1.0, 999)
                    }
                },
                _ => {}
            }
       } 
    }
}

use super::test_hero;
test_hero!(Marville);
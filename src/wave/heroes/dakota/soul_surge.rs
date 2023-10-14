use crate::{wave::{heroes::{Cooldown, Selector, Typed, Execute, Skilled}, }, data::{skill::{SkillType, Select}, effect::{Effect}, }, };


#[derive(Cooldown, Debug, PartialEq, Deserialize, Serialize, Clone,Copy )]
pub struct SoulSurge {
    pub cooldown : u32,
    pub toxic_swamp_turns : u32,
    pub rose_poison_chance : f32,
    pub speed_up_turns : u32,
}

impl Default for SoulSurge {
    fn default() -> Self {
        Self {
            cooldown : 3,
            toxic_swamp_turns : 3,
            rose_poison_chance : 1.,
            speed_up_turns : 2,
        }
    }
}

impl Selector for SoulSurge {
    const SELECT :Select = Select::AllAllies;
}
impl Typed for SoulSurge {
    const TYPE : SkillType  = SkillType::Active; 
}
impl Execute for SoulSurge {
    fn execute(&self, wave : &mut crate::wave::Wave<'_>, _skill : &crate::data::skill::Skill, actor : crate::wave::InstanceIndex, _target : crate::wave::InstanceIndex) {
                wave.inflict_enemy_team(actor, Effect::ToxicSwamp, 1.0, self.toxic_swamp_turns);
                wave.inflict_ally_team(actor, Effect::SpeedUpII, 1.0, self.speed_up_turns);
    }
}
impl Skilled for SoulSurge {}
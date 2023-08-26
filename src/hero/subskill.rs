use crate::wave::{Wave, InstanceIndex};

use super::effect::Effect;

#[derive(Deserialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Target {
    Everyone,
    SingleAlly,
    SingleEnemy,
    AllEnemies,
    AllAllies,
}
#[derive(Deserialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Scale {
    AttackDamage,
    MaxHealth,
}
#[derive(Deserialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Type {
    Damage,
    Restore,
    Inflict
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct SubSkill {
    pub target : Target,
    #[serde(default="ratio_default")]
    pub ratio : f32,
    #[serde(rename="type")]
    pub typ : Type,
    pub scale : Scale,
    pub effect : Effect,
    #[serde(default="chance_default")]
    pub chance : f32,
    #[serde(default="turns_default")]
    pub turns : u32,
}

fn ratio_default() -> f32 {
    1.0
}
fn chance_default() -> f32 {
    0.0
}
fn turns_default() -> u32 {
    0
}

pub fn execute_subskill<const LEN:usize>(subskill : &SubSkill, actor :InstanceIndex, target :InstanceIndex, wave :&mut Wave<LEN>) {
    let mut val= 0.0;
    let mut targets : Vec<InstanceIndex> = vec![];
    let mut effect = Effect::None;
    let mut chance = 0.0;
    let mut turns = 0;
    match subskill.scale {
        Scale::AttackDamage => {
            val= wave.get_attack_damage(actor) * subskill.ratio;
        },
        Scale::MaxHealth => {
            val= wave.get_max_health(actor) * subskill.ratio;
        },
    }
    match subskill.effect {
        Effect::None => {},
        _ => {
            effect = subskill.effect;
            chance = subskill.chance;
            turns = subskill.turns;
        },
    }
    match subskill.target {
        Target::Everyone => {
            // 0..LEN
            targets = wave.get_indices();
        },
        Target::SingleAlly => {
            targets  = vec![target];
        },
        Target::SingleEnemy => {
            targets  = vec![target];
        },
        Target::AllEnemies => {
            targets = wave.get_enemies_indices(actor);
        },
        Target::AllAllies => {
            targets = wave.get_ally_indices(actor);
        },
    } 
    match subskill.typ {
        Type::Damage => {
            for target in targets {
                wave.attack_single(actor,target,val);
            }
        },
        Type::Restore => {
            for target in targets {
                wave.restore_single(actor,target,val);
            }
        },
        Type::Inflict => {
            for target in targets {
                wave.inflict_single(actor,target,effect,chance,turns);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_xml_attr() {
        let hero: SubSkill= serde_xml_rs::from_str(
            r#"
            <subskill target="SingleEnemy" type="Damage" ratio="2.0" scale="AttackDamage" effect="WetI" chance="0.0" turns="0" />
            "#,
        )
        .unwrap();

        assert_eq!(hero.target, Target::SingleEnemy);
        assert_eq!(hero.typ, Type::Damage);
        assert_eq!(hero.ratio, 2.0);
        assert_eq!(hero.scale, Scale::AttackDamage);
        assert_eq!(hero.effect, Effect::WetI);
        assert_eq!(hero.chance, 0.0);
    }

    #[test]
    fn read_xml() {
        let hero: SubSkill= serde_xml_rs::from_str(
            r#"
            <subskill>
                <target>SingleEnemy</target>
                <type>Damage</type>
                <ratio>1.0</ratio>
                <scale>AttackDamage</scale>
                <effect>WetI</effect>
                <chance>0.0</chance>
                <turns>0</turns>
            </subskill>
            "#,
        )
        .unwrap();

        assert_eq!(hero.target, Target::SingleEnemy);
        assert_eq!(hero.typ, Type::Damage);
        assert_eq!(hero.ratio, 1.0);
        assert_eq!(hero.scale, Scale::AttackDamage);
        assert_eq!(hero.effect, Effect::WetI);
        assert_eq!(hero.chance, 0.0);
    }
}

use crate::wave::{Wave, InstanceIndex};

use super::{effect::Effect, skill::Skill};
use quick_xml::de::from_str;
use quick_xml::se::to_string;

#[derive(Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Target {
    Everyone,
    SingleAlly,
    SingleEnemy,
    AllEnemies,
    AllAllies,
    SingleSelf,
    LowestHealthAlly,
    None,
}

//pub fn get_subskill_targets<const LEN:usize>(target : Target, actor :InstanceIndex, wave :& Wave<LEN>) -> Vec<InstanceIndex> {
//    match target {
//        Target::Everyone => {
//            // 0..LEN
//            wave.get_indices()
//        },
//        Target::SingleAlly => {
//            wave.get_ally_indices(actor)
//        },
//        Target::SingleEnemy => {
//            wave.get_enemies_indices(actor)
//        },
//        Target::AllEnemies => {
//            wave.get_enemies_indices(actor)
//        },
//        Target::AllAllies => {
//            wave.get_ally_indices(actor)
//        },
//        Target::SingleSelf => {
//            vec![actor]
//        },
//        Target::None => {
//            vec![]
//        }
//        Target::LowestHealthAlly => {
//            vec![wave.get_lowest_health_ally(actor)]
//        }
//    } 
//}

pub fn merge_targets(t1 : Target,t2:Target) -> Target {
    // test if it makes sense
    if (Target::SingleAlly == t1 || Target::AllAllies== t1) && (Target::SingleEnemy == t2 || Target::AllEnemies == t2) {
        panic!("Cannot merge targets {:?} and {:?}",t1,t2);
    }
    if (Target::SingleEnemy == t1 || Target::AllEnemies== t1) && (Target::SingleAlly == t2 || Target::AllAllies == t2) {
        panic!("Cannot merge targets {:?} and {:?}",t1,t2);
    }
    // return 
    if Target::SingleAlly == t1 || Target::SingleAlly == t2 {
        t1
    } else if Target::SingleEnemy == t1 || Target::SingleEnemy == t2 {
        t1
    } else if Target::AllEnemies == t1 || Target::AllEnemies == t2 {
        t1
    } else if Target::AllAllies == t1 || Target::AllAllies == t2 {
        t1
    } else if Target::Everyone == t1 || Target::Everyone == t2 {
        t1
    }
    else {
        panic!("Cannot merge targets {:?} and {:?}",t1,t2);
    }
}

#[derive(Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Scale {
    AttackDamage,
    MaxHealth,
    TargetMaxHealth,
    None,
}
#[derive(Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Type {
    Damage,
    Restore,
    Inflict,
    RemoveAllBuffs,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct SubSkill {
    #[serde(default="target_default", rename= "@target")]
    pub target : Target,
    #[serde(default="ratio_default", rename= "@ratio")]
    pub ratio : f32,
    #[serde(rename="@type")]
    pub typ : Type,
    #[serde(default="scale_default",rename="@scale")]
    pub scale : Scale,
    #[serde(default="effect_default",rename= "@effect")]
    pub effect : Effect,
    #[serde(default="chance_default",rename= "@chance")]
    pub chance : f32,
    #[serde(default="turns_default",rename= "@turns")]
    pub turns : u32,
}
fn target_default() -> Target {
    Target::SingleEnemy
}
fn scale_default() -> Scale{
    Scale::None
}
fn effect_default() -> Effect{
    Effect::None
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


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn write_xml() {
        let ss = SubSkill {
            target : Target::SingleEnemy,
            ratio : 1.0,
            typ : Type::Damage,
            scale : Scale::AttackDamage,
            effect : Effect::WetI,
            chance : 0.0,
            turns : 0,
        };

        let xml = to_string(&ss).unwrap();
        //panic!("{}",xml);
    }

    #[test]
    fn read_xml() {
        let hero: SubSkill= from_str(
            r#"
            <subskill target="SingleEnemy" type="Damage" ratio="1.0" scale="AttackDamage" effect="WetI" chance="0.0" turns="0" />
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

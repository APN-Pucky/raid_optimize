
use strum_macros::EnumIter;

use super::{effect::Effect};



#[derive(EnumIter,strum_macros::Display,Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Target {
    None,
    Everyone,
    SingleAlly,
    SingleEnemy,
    AllEnemies,
    AllAllies,
    SingleSelf,
    LowestHealthAlly,
}


#[derive(EnumIter,strum_macros::Display,Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Scale {
    None,
    Attack,
    Defense,
    MaxHealth,
    TargetMaxHealth,
}
#[derive(EnumIter,strum_macros::Display,Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Type {
    Damage,
    Restore,
    Inflict,
    RemoveAllBuffs,
    Shield,
    ReduceTurnMeter,
    IncreaseTurnMeter,
}

#[derive(EnumIter,strum_macros::Display,Deserialize, Serialize, Debug, Clone,Eq, PartialEq,Copy)]
pub enum Trigger {
    None,
    BeginningOfEachTurn,
    EnemyDeath,
    AllyDeath,
    //SelfDeath
    AnyDeath
}


#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct SubSkill {
    #[serde(default="target_default", rename= "@target")]
    pub target : Target,
    #[serde(default="ratio_default", rename= "@ratio")]
    pub ratio : f32,
    #[serde(default="typ_default",rename="@type")]
    pub typ : Type,
    #[serde(default="scale_default",rename="@scale")]
    pub scale : Scale,
    #[serde(default="effect_default",rename= "@effect")]
    pub effect : Effect,
    #[serde(default="chance_default",rename= "@chance")]
    pub chance : f32,
    #[serde(default="turns_default",rename= "@turns")]
    pub turns : u32,
    #[serde(default="trigger_default",rename= "@trigger")]
    pub trigger: Trigger,
}

impl Default for SubSkill {
    fn default() -> Self {
        Self {
            target : target_default(),
            ratio : ratio_default(),
            typ : typ_default(),
            scale : scale_default(),
            effect : effect_default(),
            chance : chance_default(),
            turns : turns_default(),
            trigger : trigger_default(),
        }
    }
}

fn trigger_default() -> Trigger {
    Trigger::None
}

fn typ_default() -> Type {
    Type::Damage
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


#[cfg(test)]
mod tests {
    use quick_xml::{se::to_string, de::from_str};

    use super::*;



    #[test]
    fn write_xml() {
        let ss = SubSkill {
            target : Target::SingleEnemy,
            ratio : 1.0,
            typ : Type::Damage,
            scale : Scale::Attack,
            effect : Effect::WetI,
            chance : 0.0,
            turns : 0,
            trigger : Trigger::None
        };

        let _xml = to_string(&ss).unwrap();
        //panic!("{}",xml);
    }

    #[test]
    fn read_xml() {
        let hero: SubSkill= from_str(
            r#"
            <subskill target="SingleEnemy" type="Damage" ratio="1.0" scale="Attack" effect="WetI" chance="0.0" turns="0" />
            "#,
        )
        .unwrap();

        assert_eq!(hero.target, Target::SingleEnemy);
        assert_eq!(hero.typ, Type::Damage);
        assert_eq!(hero.ratio, 1.0);
        assert_eq!(hero.scale, Scale::Attack);
        assert_eq!(hero.effect, Effect::WetI);
        assert_eq!(hero.chance, 0.0);
    }
}

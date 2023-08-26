use super::effects::Effect;

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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_xml_attr() {
        let hero: SubSkill= serde_xml_rs::from_str(
            r#"
            <subskill target="SingleEnemy" ratio="2.0" scale="AttackDamage" effect="WetI" chance="0.0" turns="0" />
            "#,
        )
        .unwrap();

        assert_eq!(hero.target, Target::SingleEnemy);
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
        assert_eq!(hero.ratio, 1.0);
        assert_eq!(hero.scale, Scale::AttackDamage);
        assert_eq!(hero.effect, Effect::WetI);
        assert_eq!(hero.chance, 0.0);
    }
}

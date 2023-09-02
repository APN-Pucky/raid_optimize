use std::fmt;

use super::{mark::Mark, class::Class, faction::Faction, rarity::Rarity, skill::Skill, passive::Passive, heroes::Heroes};



#[derive(Deserialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    pub mark : Mark,
    pub class: Class,
    pub faction: Faction,
    pub rarity : Rarity,
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub effect_hit: f32,
    pub effect_resistance: f32,
    pub mastery: f32,
    pub healing_effect: f32,
    pub leech: f32,
    pub piercing: f32,
    pub tenacity: f32,
    pub damage_reflection : f32,
    #[serde(rename="skill")]
    pub skills: Vec<Skill>,
    #[serde(rename="passive")]
    pub passives : Vec<Passive>,
}

pub fn get_hero_by_string<'a>(heroes: &'a Heroes, name: &'a str) -> Option<&'a Hero> {
    for hero in heroes.heroes.iter() {
        if hero.name == name  || name  == format!("[{}]", hero.id) 
        { // TOOD handle multiple names
            return Some(hero)
        }
    }
    None
    //panic!("Hero not found: {}", name);
}


impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (health: {}, attack: {})", self.name, self.health, self.attack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_xml_file() {
        let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
        let heroes : Heroes = serde_xml_rs::from_str(&file_string).unwrap();
        assert_eq!(heroes.heroes[0].id, 1);
    }

    #[test]
    fn read_xml() {
        let hero: Hero = serde_xml_rs::from_str(
            r#"
            <hero>
                <id>1</id>
                <name>Elhain</name>
                <rarity>Legendary</rarity>
                <mark>Blue</mark>
                <class>Support</class>
                <faction>WizardsEye</faction>
                <health>15000</health>
                <attack>1000</attack>
                <defense>1000</defense>
                <speed>100</speed>
                <crit_rate>0.15</crit_rate>
                <crit_damage>1.5</crit_damage>
                <effect_hit>0.15</effect_hit>
                <effect_resistance>0.15</effect_resistance>
                <mastery>0.15</mastery>
                <healing_effect>0.15</healing_effect>
                <leech>0.15</leech>
                <piercing>0.15</piercing>
                <tenacity>0.15</tenacity>
                <damage_reflection>0.15</damage_reflection>
                <passive>
                    <BloodthirstyDesire />
                </passive>
                <skill>
                    <ScorchedSoul>
                        <cooldown>3</cooldown>
                        <attack_damage_ratio>1.0</attack_damage_ratio>
                        <hp_burning_chance>0.5</hp_burning_chance>
                        <hp_burning_turns>2</hp_burning_turns>
                    </ScorchedSoul>
                </skill>
                <skill>
                    <Generic>
                        <cooldown>3</cooldown>
                        <name>test</name>
                        <subskill target="SingleEnemy" type="Damage" ratio="2.0" scale="AttackDamage" effect="WetI" chance="0.0" turns="0" />
                    </Generic>
                </skill>
            </hero>
            "#,
        )
        .unwrap();

        assert_eq!(hero.id, 1);
        assert_eq!(hero.name, "Elhain");
        assert_eq!(hero.health, 15000.);
        assert_eq!(hero.attack, 1000.);
        assert_eq!(hero.defense, 1000.);
        assert_eq!(hero.speed, 100.);
        assert_eq!(hero.crit_rate, 0.15);
        assert_eq!(hero.crit_damage, 1.5);
        assert_eq!(hero.effect_hit, 0.15);
        assert_eq!(hero.effect_resistance, 0.15);
        assert_eq!(hero.mastery, 0.15);
        assert_eq!(hero.healing_effect, 0.15);
        assert_eq!(hero.leech, 0.15);
        assert_eq!(hero.piercing, 0.15);
        assert_eq!(hero.tenacity, 0.15);
    }
}
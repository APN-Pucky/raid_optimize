use std::fmt;

use super::{mark::Mark, class::Class, faction::Faction, rarity::Rarity, skill::Skill, heroes::Heroes};




#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub mark : Mark,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub class: Class,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub faction: Faction,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
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
    //#[serde(rename="passive")]
    //pub passives : Vec<Passive>,
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            id: 0,
            name: "".to_string(),
            mark : Mark::Blue,
            class: Class::Attack,
            faction: Faction::DoomLegion,
            rarity : Rarity::Legendary,
            health: 0.,
            attack: 0.,
            defense: 0.,
            speed: 0.,
            crit_rate: 0.,
            crit_damage: 0.,
            effect_hit: 0.,
            effect_resistance: 0.,
            mastery: 0.,
            healing_effect: 0.,
            leech: 0.,
            piercing: 0.,
            tenacity: 0.,
            damage_reflection : 0.,
            skills: vec![],
        }
    }
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
    use itertools::assert_equal;
    use quick_xml::{se::to_string, de::from_str};

    use super::*;

    #[test]
    fn write_xml() {
        

        let hero = Hero{
            id: 1,
            name: "Elhain".to_string(),
            mark : Mark::Blue,
            class: Class::Support,
            faction: Faction::WizardsEye,
            rarity : Rarity::Legendary,
            health: 15000.,
            attack: 1000.,
            defense: 1000.,
            speed: 100.,
            crit_rate: 0.15,
            crit_damage: 1.5,
            effect_hit: 0.15,
            effect_resistance: 0.15,
            mastery: 0.15,
            healing_effect: 0.15,
            leech: 0.15,
            piercing: 0.15,
            tenacity: 0.15,
            damage_reflection : 0.15,
            skills: vec![],
        };
        //String buffer writter
        //let mut buffer = Vec::new();
        
        //match serde_xml_rs::to_writer(&mut buffer,&hero)  {
        //    Ok(_) => {},
        //    Err(e) => println!("Error: {}", e),
        //};
        //let xml = String::from_utf8(buffer).unwrap();
        let xml = to_string(&hero).unwrap();
        //panic!("{}", xml);
        /* 
        assert_eq!(xml,            r#"
        <hero>
            <id>1</id>
            <name>Elhain</name>
            <mark>Blue</mark>
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
            <skill>
            </skill>
        </hero>"#.to_string() );
        */
    }

    #[test]
    fn read_xml() {
        let hero: Hero = from_str(
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
                <skill>
                    <type>Passive</type>
                    <BloodthirstyDesire />
                </skill>
                <skill>
                    <cooldown>3</cooldown>
                    <ScorchedSoul>
                        <attack_damage_ratio>1.0</attack_damage_ratio>
                        <hp_burning_chance>0.5</hp_burning_chance>
                        <hp_burning_turns>2</hp_burning_turns>
                    </ScorchedSoul>
                </skill>
                <skill>
                    <cooldown>3</cooldown>
                    <Generic>
                        <name>test</name>
                        <subskill target="SingleEnemy" type="Damage" ratio="1.0" scale="AttackDamage" effect="WetI" chance="0.0" turns="0" />
                    </Generic>
                </skill>
            </hero>
            "#)

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

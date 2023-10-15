use std::fmt;

use super::{
    class::Class, faction::Faction, heroes::Heroes, mark::Mark, rarity::Rarity, skill::Skill,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub mark: Mark,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub class: Class,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub faction: Faction,
    #[serde(with = "quick_xml::serde_helpers::text_content")]
    pub rarity: Rarity,
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
    pub damage_reflection: f32,
    #[serde(rename = "$value", default)]
    pub skills: Vec<Skill>,
    //#[serde(rename="passive")]
    //pub passives : Vec<Passive>,
}

impl Default for Hero {
    fn default() -> Self {
        Hero {
            id: 0,
            name: "".to_string(),
            mark: Mark::Blue,
            class: Class::Attack,
            faction: Faction::DoomLegion,
            rarity: Rarity::Legend,
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
            damage_reflection: 0.,
            skills: vec![],
        }
    }
}

pub fn get_hero_by_string<'a>(heroes: &'a Heroes, name: &'a str) -> Option<&'a Hero> {
    for hero in heroes.heroes.iter() {
        if hero.name == name || name == format!("[{}]", hero.id) {
            // TOOD handle multiple names
            return Some(hero);
        }
    }
    None
    //panic!("Hero not found: {}", name);
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (health: {}, attack: {})",
            self.name, self.health, self.attack
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::wave::heroes::natalie::bloodthristy_desire::BloodthirstyDesire;

    use quick_xml::{de::from_str, se::to_string};

    use super::*;

    #[test]
    fn write_xml() {
        let hero = Hero {
            id: 1,
            name: "Elhain".to_string(),
            mark: Mark::Blue,
            class: Class::Support,
            faction: Faction::WizardsEye,
            rarity: Rarity::Legend,
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
            damage_reflection: 0.15,
            skills: vec![Skill::BloodthirstyDesire(BloodthirstyDesire::default())],
        };
        //String buffer writter
        //let mut buffer = Vec::new();

        //match serde_xml_rs::to_writer(&mut buffer,&hero)  {
        //    Ok(_) => {},
        //    Err(e) => println!("Error: {}", e),
        //};
        //let xml = String::from_utf8(buffer).unwrap();
        let _xml = to_string(&hero).unwrap();
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
                <rarity>Legend</rarity>
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
                <BloodthirstyDesire />
                <ScorchedSoul>
                    <cooldown>3</cooldown>
                    <attack_damage_ratio>1.0</attack_damage_ratio>
                    <hp_burning_chance>0.5</hp_burning_chance>
                    <hp_burning_turns>2</hp_burning_turns>
                </ScorchedSoul>
                <Generic>
                    <cooldown>3</cooldown>
                    <name>test</name>
                    <subskill target="SingleEnemy" type="Damage" ratio="1.0" scale="Attack" effect="WetI" chance="0.0" turns="0" />
                </Generic>
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
        assert_eq!(hero.skills.len(), 3);
    }
}

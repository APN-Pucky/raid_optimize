use std::fmt;

pub mod instance;
pub mod statistics;

#[derive(Deserialize, Debug)]
pub struct Heroes {
    #[serde(rename="hero")]
    pub heroes: Vec<Hero>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub effect_hit: f32,
    pub effect_resistance: f32,
    pub mastery: f32,
    pub healing_effect: f32,
    pub leech: f32,
    pub piercing: f32,
    pub tenacity: f32,
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
            </hero>
            "#,
        )
        .unwrap();

        assert_eq!(hero.id, 1);
        assert_eq!(hero.name, "Elhain");
        assert_eq!(hero.health, 15000);
        assert_eq!(hero.attack, 1000);
        assert_eq!(hero.defense, 1000);
        assert_eq!(hero.speed, 100);
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

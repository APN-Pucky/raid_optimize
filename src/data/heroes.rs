use super::hero::Hero;


#[derive(Deserialize, Debug)]
pub struct Heroes {
    #[serde(rename="hero")]
    pub heroes: Vec<Hero>,
}
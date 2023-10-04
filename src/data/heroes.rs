use super::hero::Hero;




#[derive(Deserialize, Serialize, Debug)]
pub struct Heroes {
    #[serde(rename="hero")]
    pub heroes: Vec<Hero>,
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;
    use quick_xml::de::from_str;

    use crate::data::load_heroes;

    use super::*;

    #[test]
    fn read_xml_file() {
        let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
        let heroes : Heroes = from_str(&file_string).unwrap();
        assert_eq!(heroes.heroes[0].id, 1);
    }

    #[test]
    fn write_xml_file() {
        //let heroes = load_heroes("data/heroes.xml".to_string());
        //let mut buffer = Vec::new();
        //match serde_xml_rs::to_writer(&mut buffer,&heroes)  {
        //    Ok(_) => {},
        //    Err(e) => println!("Error: {}", e),
        //};
        //let xml = String::from_utf8(buffer).unwrap();
        //assert_eq!(xml,            r#""#);
    }
}
use log::debug;
use log::error;
use log::info;
use log::warn;

use raid_optimize::test;
use raid_optimize::hero::Heroes;
use raid_optimize::wave::Wave;

pub fn main() {
    env_logger::init();
    let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
    let heroes : Heroes = serde_xml_rs::from_str(&file_string).unwrap();
    let ally = vec![&heroes.heroes[0]];
    let enemy = vec![&heroes.heroes[1]];
    let mut wave = Wave::new(
        &ally,
        &enemy
    );
    wave.simulate();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        main();
    }
}

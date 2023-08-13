use log::debug;
use log::error;
use log::info;
use log::warn;

use raid_optimize::test;
use raid_optimize::hero::Heroes;
use raid_optimize::sim::Sim;

pub fn main() {
    env_logger::init();
    let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
    let heroes : Heroes = serde_xml_rs::from_str(&file_string).unwrap();
    let ally = vec![&heroes.heroes[0]];
    let enemy = vec![&heroes.heroes[1]];
    let mut wave = Sim::new(
        &ally,
        &enemy
    );
    wave.run();
    wave.print_results();
    wave.print_statistics();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        main();
    }
}

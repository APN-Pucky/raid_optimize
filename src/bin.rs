extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

use log::debug;
use log::error;
use log::info;
use log::warn;

use raid_optimize::test;
use raid_optimize::hero::Heroes;
use raid_optimize::sim::Sim;
use raid_optimize::hero::get_hero_by_string;

pub fn main() {
    env_logger::init();

    let mut str_ally = String::new();
    let mut str_enemy = String::new();
    let mut iterations = 10000;
    let mut threads = 1;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Raid optimizer");
        ap.refer(&mut str_ally)
            .add_option(&["-a", "--ally"], Store, "Ally team").required();
        ap.refer(&mut str_enemy)
            .add_option(&["-e", "--enemy"], Store, "Enemy team").required();
        ap.refer(&mut iterations)
            .add_option(&["-i", "--iterations"], Store, "Number of iterations").required();
        ap.refer(&mut threads)
            .add_option(&["-t", "--threads"], Store, "Number of threads");
        ap.parse_args_or_exit();
    }
    // parse heros from xml
    let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
    let heroes : Heroes = serde_xml_rs::from_str(&file_string).unwrap();
    // parse comma separated list of units in each team
    let mut ally = vec![];
    let mut enemy = vec![];

    for name in str_ally.split(",").map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Ally: {}", h);
                ally.push(h);
            },
            None => {
                warn!("Hero not found: {}", str_ally);
            }
        }
    }
    for name in str_enemy.split(",").map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Enemy: {}", h);
                enemy.push(h);
            },
            None => {
                warn!("Hero not found: {}", str_enemy);
            }
        }
    }

    let mut wave = Sim::new(
        &ally,
        &enemy,
        iterations
    );
    wave.run(threads);
    wave.print_results();
    wave.print_statistics();
}
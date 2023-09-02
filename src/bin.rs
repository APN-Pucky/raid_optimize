extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

use log::info;
use log::warn;

use raid_optimize::data::hero::get_hero_by_string;
use raid_optimize::data::heroes::Heroes;
use raid_optimize::sim;
use raid_optimize::sim::Sim;

pub fn main() {
    env_logger::init();

    let mut str_ally = String::new();
    let mut str_enemy = String::new();
    let mut iterations : u64 = 10000;
    let mut threads= 1;
    let mut print_version = false;
    let mut no_stats = false;
    let mut no_results = false;
    let mut bar = false;
    let mut manual_ally = false;
    let mut manual_enemy = false;

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
        // print version
        ap.refer(&mut print_version)
            .add_option(&["-v", "--version"],
            StoreTrue,
            "Print version");
        ap.refer(&mut no_stats)
            .add_option(&["--no-stats"],
            StoreTrue,
            "Don't print statistics");
        ap.refer(&mut no_results)
            .add_option(&["--no-results"],
            StoreTrue,
            "Don't print results");
        ap.refer(&mut bar)
            .add_option(&["-b", "--bar"], StoreTrue, "Bar");
        ap.refer(&mut manual_ally)
            .add_option(&["--manual-ally"], StoreTrue, "Bar");
        ap.refer(&mut manual_enemy)
            .add_option(&["--manual-enemy"], StoreTrue, "Bar");
        ap.parse_args_or_exit();
    }
    if print_version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    // parse heros from xml
    let file_string = std::fs::read_to_string("data/heroes.xml").unwrap();
    let heroes : Heroes = serde_xml_rs::from_str(&file_string).unwrap();
    // parse comma separated list of units in each team
    let mut ally = vec![];
    let mut enemy = vec![];

    for name in str_ally.split(',').map(|s| s.trim()) {
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
    for name in str_enemy.split(',').map(|s| s.trim()) {
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
    let len : usize = ally.len() + enemy.len();

macro_rules! sim {
    ($l:tt) => { 
        const LEN : usize = $l;
        let mut sim : Sim<'_,LEN> = Sim::new(
            &ally,
            &enemy,
            manual_ally,
            manual_enemy,
            iterations,
        );
        sim.run(threads,!no_stats);
        if !no_stats {
            sim.print_statistics(bar);
        }
        if !no_results{
            sim.print_results();
        }
     }
}

    // speed up routines by known size and array on stack
    match len {
        1 => {
            sim!(1);
        }
        2 => {
            sim!(2);
        }
        3 => {
            sim!(3);
        }
        4 => {
            sim!(4);
        }
        5 => {
            sim!(5);
        }
        6 => {
            sim!(6);
        }
        7 => {
            sim!(7);
        }
        8 => {
            sim!(8);
        }
        9 => {
            sim!(9);
        }
        10 => {
            sim!(10);
        }
        _ => {
            panic!("too large team size: {}", len);
        }
    }


}
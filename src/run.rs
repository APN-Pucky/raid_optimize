

use log::info;
use log::warn;

use raid_optimize::data::hero::get_hero_by_string;
use raid_optimize::data::heroes::Heroes;
use raid_optimize::sim;
use raid_optimize::sim::Sim;
use raid_optimize::input::Input;

pub fn run(input : Input) {
    if input.print_version {
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
                debug!("Ally: {}", h);
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
                debug!("Enemy: {}", h);
                enemy.push(h);
            },
            None => {
                warn!("Hero not found: {}", str_enemy);
            }
        }
    }
    let len : usize = ally.len() + enemy.len();

    let mut sim : Sim<'_> = Sim::new(
            &ally,
            &enemy,
            intput.manual_ally,
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

macro_rules! sim {
    ($l:tt) => { 
        const LEN : usize = $l;
        
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
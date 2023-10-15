use crate::{
    data::{
        hero::{get_hero_by_string, Hero},
        load_heroes,
    },
    input::Input,
};

use log::info;
use log::warn;

#[derive(Debug, Clone)]
pub struct Args {
    //pub heroes : Heroes,
    pub allies: Vec<Hero>,
    pub enemies: Vec<Hero>,
    pub iterations: u64,
    pub threads: u64,
    //pub print_version : bool,
    pub stats: bool,
    //pub no_results    : bool,
    //pub bar           : bool,
    pub manual_ally: bool,
    pub manual_enemy: bool,

    pub turn_limit: u32,
    pub turn_meter_threshold: f32,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            //heroes : Heroes::default(),
            allies: vec![],
            enemies: vec![],
            iterations: 1000,
            threads: 1,
            //print_version : false,
            stats: true,
            //no_results : false,
            //bar : false,
            manual_ally: false,
            manual_enemy: false,
            turn_limit: 300,
            turn_meter_threshold: 1000.0,
        }
    }
}

pub fn args_from_input(input: Input) -> Args {
    let heroes = load_heroes(input.heroes_xml);
    let mut args = Args {
        allies: Vec::new(),
        enemies: Vec::new(),
        iterations: input.iterations,
        threads: input.threads,
        //print_version : input.print_version,
        stats: !input.no_stats,
        //no_results : input.no_results,
        //bar : input.bar,
        manual_ally: input.manual_ally,
        manual_enemy: input.manual_enemy,
        turn_limit: 300,
        turn_meter_threshold: 1000.,
    };

    for name in input.allies.split(',').map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Ally: {}", h);
                args.allies.push(h.clone());
            }
            None => {
                warn!("Hero not found: {}", name);
            }
        }
    }
    for name in input.enemies.split(',').map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Enemy: {}", h);
                args.enemies.push(h.clone());
            }
            None => {
                warn!("Hero not found: {}", name);
            }
        }
    }
    args
}

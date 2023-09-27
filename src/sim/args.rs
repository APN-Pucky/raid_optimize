use crate::{data::{hero::{Hero, get_hero_by_string}, load_heroes, heroes::Heroes}, input::Input};

use log::info;
use log::warn;

pub struct Args {
    //pub heroes : Heroes,
    pub allies : Vec<Hero>,
    pub enemies: Vec<Hero>,
    pub iterations : u64,
    pub threads : u64 ,
    pub print_version : bool,
    pub no_stats      : bool,
    pub no_results    : bool,
    pub bar           : bool,
    pub manual_ally   : bool,
    pub manual_enemy  : bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            //heroes : Heroes::default(),
            allies : vec![Hero::default(),Hero::default(),Hero::default(),Hero::default(),Hero::default()],
            enemies : vec![Hero::default(),Hero::default(),Hero::default(),Hero::default(),Hero::default()],
            iterations : 1000,
            threads : 1,
            print_version : false,
            no_stats : false,
            no_results : false,
            bar : false,
            manual_ally : false,
            manual_enemy : false,
        }
    }
}

pub fn args_from_input(input:Input) ->Args {
    let heroes = load_heroes(input.heroes_xml);
    let mut args = Args {
        allies : Vec::new(),
        enemies : Vec::new(),
        iterations : input.iterations,
        threads : input.threads,
        print_version : input.print_version,
        no_stats : input.no_stats,
        no_results : input.no_results,
        bar : input.bar,
        manual_ally : input.manual_ally,
        manual_enemy : input.manual_enemy,
    };

    for name in input.allies.split(',').map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Ally: {}", h);
                args.allies.push(h.clone());
            },
            None => {
                warn!("Hero not found: {}", name );
            }
        }
    }
    for name in input.enemies.split(',').map(|s| s.trim()) {
        let hero = get_hero_by_string(&heroes, name);
        match hero {
            Some(h) => {
                info!("Enemy: {}", h);
                args.enemies.push(h.clone());
            },
            None => {
                warn!("Hero not found: {}", name);
            }
        }
    }
    args
}
use rand::Rng;
use std::io;

use crate::{wave::{Wave, TeamIndex, InstanceIndex}, hero::skill::Skill};


pub trait Player<const LEN:usize> {
    fn get_name(&self) -> String;
    fn get_team(&self) -> TeamIndex;
    fn pick_skill<'a>(&self, wave : &Wave<LEN>, actor : InstanceIndex, skills: &Vec<&'a Skill>) -> &'a Skill;
    fn pick_target(&self, wave : &Wave<LEN>, actor : InstanceIndex, skill : &Skill, targets: &Vec<InstanceIndex>) -> InstanceIndex; 
}

pub struct RandomPlayer {
    pub(crate) team_index : TeamIndex 
}

impl<const LEN:usize> Player<LEN> for RandomPlayer {
    fn get_team(&self) -> TeamIndex {
        self.team_index
    }
    fn get_name(&self) -> String {
        format!("RandomPlayer {}", self.team_index)
    }
    fn pick_target(&self, _wave : &Wave<LEN>, _actor : InstanceIndex, _skill : &Skill, targets: &Vec<InstanceIndex>) -> InstanceIndex{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..targets.len());
        targets[index]
    }
    fn pick_skill<'a>(&self, _wave : &Wave<LEN>, _actor : InstanceIndex, skills: &Vec<&'a Skill>) -> &'a Skill{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..skills.len());
        skills[index]
    }
}

pub struct ManualPlayer {
    pub(crate) team_index : TeamIndex 
}

impl ManualPlayer {

    fn handle_inputs<const LEN:usize>(&self,s :&str , wave:&Wave<LEN>, actor : InstanceIndex) -> bool {
        match s {
            "s" => {
                // show status
                    wave.print_allies(actor);
                    wave.print_enemies(actor);
                true
                

            },
            "se" => {
                // show enemy status
                    wave.print_enemies(actor);
                true

            },
            "sa" => {
                // show ally status
                wave.print_allies(actor);
                true

            },
            "h" => {
                // show help
                println!("h: show help");
                println!("s: show status");
                println!("se: show enemy status");
                println!("sa: show ally status");
                println!("q: quit");
                true
            }
            "q" => {
                std::process::exit(0)
            },
            _ => {false}
        }
    }
}

impl<const LEN:usize> Player<LEN> for ManualPlayer {
    fn get_team(&self) -> TeamIndex {
        self.team_index
    }
    fn get_name(&self) -> String {
        format!("ManualPlayer {}", self.team_index)
    }
    fn pick_target(&self, wave : &Wave<LEN>, actor : InstanceIndex, skill : &Skill, targets: &Vec<InstanceIndex>) -> InstanceIndex{
        println!("Pick target for {} using skill {:?}", wave.heroes[actor].name, skill);
        for (i, target) in targets.iter().enumerate() {
            println!(" {}: {}", i, wave.name(*target));
        }
        loop {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            if ! self.handle_inputs(s.trim(),wave,actor) {
                match s.trim().parse::<usize>() {
                    Ok(index) => {
                        if index < targets.len() {
                            return targets[index];
                        }
                        else {
                            println!("Invalid target index");
                        }
                    },
                    Err(_) => {
                        println!("Invalid target index");
                    }
                }
            }

        }
    }


    fn pick_skill<'a>(&self, wave : &Wave<LEN>, actor :InstanceIndex, skills: &Vec<&'a Skill>) -> &'a Skill{
        println!("Pick skill for {}", wave.heroes[actor].name);
        for (i, skill) in skills.iter().enumerate() {
            println!(" {}: {:?}", i, skill);
        }
        loop {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            if ! self.handle_inputs(s.trim(),wave,actor) {
                match s.trim().parse::<usize>() {
                    Ok(index) => {
                        if index < skills.len() {
                            return skills[index];
                        }
                        else {
                            println!("Invalid skill index");
                        }
                    },
                    Err(_) => {
                        println!("Invalid skill index");
                    }
                }
            }
        }
    }
}
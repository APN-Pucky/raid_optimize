use rand::Rng;
use std::io;

use crate::{wave::{Wave, InstanceRef}, hero::skill::Skill};


pub trait Player {
    fn pick_skill(&self, wave : &Wave, actor : InstanceRef, skills: Vec<Skill>) -> Skill;
    fn pick_target(&self, wave : &Wave, actor : InstanceRef, skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef; 
}

pub struct RandomPlayer {}

impl Player for RandomPlayer {
    fn pick_target(&self, _wave : &Wave, _actor : InstanceRef, _skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..targets.len());
        targets[index]
    }
    fn pick_skill(&self, _wave : &Wave, _actor : InstanceRef, skills: Vec<Skill>) -> Skill{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..skills.len());
        skills[index]
    }
}

pub struct ManualPlayer {}

impl ManualPlayer {
    fn handle_inputs(&self,s :&str , wave:&Wave, actor : InstanceRef) -> bool {
        match s {
            "s" => {
                // show status
                if actor.team {
                    wave.print_allies();
                    wave.print_enemies();
                }
                else {
                    wave.print_enemies();
                    wave.print_allies();
                }
                true
                

            },
            "se" => {
                // show enemy status
                if actor.team {
                    wave.print_enemies();
                }
                else {
                    wave.print_allies();
                }
                true

            },
            "sa" => {
                // show ally status
                if actor.team {
                    wave.print_allies();
                }
                else {
                    wave.print_enemies();
                }
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

impl Player for ManualPlayer {
    fn pick_target(&self, wave : &Wave, actor : InstanceRef, skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef {
        println!("Pick target for {} using skill {:?}", wave.get_instance(actor).hero.name, skill);
        for (i, target) in targets.iter().enumerate() {
            println!(" {}: {}", i, wave.get_instance(*target));
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


    fn pick_skill(&self, wave : &Wave, actor : InstanceRef, skills: Vec<Skill>) -> Skill{
        println!("Pick skill for {}", wave.get_instance(actor).hero.name);
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
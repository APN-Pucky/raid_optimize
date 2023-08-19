use rand::Rng;
use std::io;

use crate::{wave::{Wave, InstanceRef}, hero::skill::Skill};


pub trait Player {
    fn pick_skill(&self, wave : &Wave, actor : InstanceRef, skills: Vec<Skill>) -> Skill;
    fn pick_target(&self, wave : &Wave, actor : InstanceRef, skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef; 
}

pub struct RandomPlayer {}

impl Player for RandomPlayer {
    fn pick_target(&self, wave : &Wave, actor : InstanceRef, skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..targets.len());
        targets[index]
    }
    fn pick_skill(&self, wave : &Wave, actor : InstanceRef, skills: Vec<Skill>) -> Skill{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..skills.len());
        skills[index]
    }
}

pub struct ManualPlayer {}

impl Player for ManualPlayer {
    fn pick_target(&self, wave : &Wave, actor : InstanceRef, skill : Skill, targets: Vec<InstanceRef>) -> InstanceRef {
        println!("Pick target for {} using skill {:?} [0-{}]", wave.get_instance(actor).hero.name, skill, targets.len()-1);
        loop {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
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
    fn pick_skill(&self, wave : &Wave, actor : InstanceRef, skills: Vec<Skill>) -> Skill{
        println!("Pick skill for {} [0-{}]", wave.get_instance(actor).hero.name, skills.len()-1);
        loop {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
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
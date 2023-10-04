

use dioxus::prelude::*;



use fermi::{AtomRef};



use serde::{Deserialize, Serialize};



use chrono::{DateTime, Local};




use crate::sim::{results::CombinedResult, args::Args};
pub struct Job {
    pub id : usize,
    pub name : String,
    pub status : Status,
    pub start_time : Option<DateTime<Local>>,
    pub end_time : Option<DateTime<Local>>,
    //pub run_time : Option<u64>, // TODO
    pub args : Args,
    //pub sim : Sim,
    pub result : CombinedResult,
    //pub result : tokio::task::JoinHandle<CombinedResult>,
    //pub result : Option<std::thread::JoinHandle<CombinedResult>>,
}

pub struct Result{
    //pub end_time : Option<DateTime<Local>>,
    pub result : CombinedResult,
}

pub enum JobId {
    ID(usize),
}

#[derive(Debug, PartialEq, Eq, strum_macros::Display,strum_macros::EnumIter,Deserialize, Serialize,Copy,Clone)]
pub enum Status {
    Pending,
    Running,
    Ended,
    Aborted,
    Failed,
}

pub struct RunState {
    pub jobs : Vec<Job>
}

impl RunState {
    pub const fn new() -> Self {
        Self {
            jobs: vec![],
        }
    }
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            jobs: vec![],
        }
    }
}

pub static RUN_STATE: AtomRef<RunState> = fermi::AtomRef(|_| RunState::default());
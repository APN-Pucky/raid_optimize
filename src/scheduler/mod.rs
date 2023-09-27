use chrono::{Utc, DateTime};
use dioxus::prelude::UseFuture;
use fermi::Atom;

use crate::sim::{Sim, results::CombinedResult, args::Args};

// pause
// stop
// start
#[derive(Clone)]
pub struct Job {
    pub id : usize,
    pub name : String,
    pub start_time : Option<DateTime<Utc>>,
    //pub end_time : Option<DateTime<Utc>>,
    //pub run_time : Option<u64>, // TODO
    pub args : Args,
    //pub sim : Sim,
    //pub result : UseFuture<CombinedResult>,
    //pub result :Option<tokio::task::JoinHandle<CombinedResult>>,
    //pub result :Option<std::thread::JoinHandle<JobResult<CombinedResult>>>,
}

#[derive(Debug,Clone)]
pub struct JobResult<T> {
    pub start_time : Option<DateTime<Utc>>,
    pub end_time : Option<DateTime<Utc>>,
    pub result : T,
}

pub fn get(job : &mut Job) {
    if let Some(res) = job.result {
        if res.is_finished() {
            job.result = None;
            job.start_time = None;
            job.result = Some(res);
        }

    }
}


#[derive(Debug,Clone)]
pub struct Scheduler {
    pub jobs : Vec<Job>,
    pub results : Vec<JobResult<CombinedResult>>,
}

//impl Default for Scheduler<'_> {
//    fn default() -> Self {
//        Self {
//            jobs: vec![],
//        }
//    }
//}


pub fn start_job(scheduler : &mut Scheduler, job : &Job) {
    let args = job.args.clone();
    Some(
        std::thread::spawn(|| {
            let mut ret:JobResult<CombinedResult> = JobResult {
                start_time : None,
                end_time : None,
                result : CombinedResult::default(),
            };
            ret.start_time = Some(Utc::now());
            let sim = Sim::new(args);
            let result = sim.run();
            ret.result = result;
            ret
        }));
    //scheduler.jobs.push(job);
}

pub static SCHEDULER : Atom<Scheduler> = Atom(|_| Scheduler {
    jobs: vec![],
    results: vec![],
});

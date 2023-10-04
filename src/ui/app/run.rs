use axum::{extract::{ws::WebSocketUpgrade, self,Path}, response::Html, routing::get, Router, Extension, Server};
use axum::http::Uri;
use dioxus::prelude::*;
use axum::extract::Host;
use axum::body::Body;
use axum::http::Request;
use fermi::{Atom, AtomRef};
use serde_json::json;
use std::sync::Mutex;
use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::{Error, Client, Response};
use dioxus_router::prelude::*;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use std::future::Future;
use std::thread::JoinHandle;
use std::future::IntoFuture;

use crate::sim::{Sim, results::CombinedResult, args::Args};
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
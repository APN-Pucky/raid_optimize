use axum::{extract::{ws::WebSocketUpgrade, self,Path}, response::Html, routing::get, Router, Extension, Server};
use axum::http::Uri;
use dioxus::prelude::*;
use axum::extract::Host;
use axum::body::Body;
use axum::http::Request;
use serde_json::json;
use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::{Error, Client, Response};
use dioxus_router::prelude::*;
use std::collections::HashMap;

pub struct Job {
    pub name : String,
    pub status : String,
    pub progress : f32,
    pub eta : f32,
    pub speed : f32,
    pub result : String,
    pub error : String,
}

pub struct RunState {
    pub jobs : Vec<Job>
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            jobs: vec![],
        }
    }
}

fn sort_by_name(a: &Job, b: &Job) -> std::cmp::Ordering {
    a.name.cmp(&b.name)
}

#[inline_props]
pub(crate) fn Run(cx: Scope) -> Element {
    let run = use_shared_state::<RunState>(cx).unwrap();
    render! {
        h2 {"Jobs"}
        div {
            // table of jobs 
            table {
                tr {
                    th { "Name" }
                    th { "Status" }
                    th { "Progress" }
                    th { "ETA" }
                    th { "Speed" }
                    th { "Result" }
                    th { "Error" }
                }
            for job in run.read().jobs.iter() {
                tr {
                    td { "{job.name}" }
                    td { "{job.status}" }
                    td { 
                        progress {
                            value: "{job.progress}",
                            max: "100",
                        }
                    }
                    td { "{job.eta}" }
                    td { "{job.speed}" }
                    td { "{job.result}" }
                    td { "{job.error}" }
                }
            }
        }
        }
    }
}
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

use crate::ui::app::run::{Job, RunState};

pub struct StartState {
    ally_team: String,
    enemy_team: String,
    flags : String,
}

impl Default for StartState {
    fn default() -> Self {
        Self {
            ally_team: "".to_string(),
            enemy_team: "".to_string(),
            flags: "".to_string(),
        }
    }
}


#[inline_props]
pub(crate) fn Start(cx: Scope) -> Element {
    let start = use_shared_state::<StartState>(cx).unwrap();
    render! {
        h2 {"Input"}
        div {
            p { "Ally Team: " }
            input {
                value: "{start.read().ally_team}",
                oninput: move |e| start.write().ally_team = e.value.clone(),
            }
        }
        div {
            p { "Enemy Team: " }
            input {
                value: "{start.read().enemy_team}",
                oninput: move |e| start.write().enemy_team = e.value.clone(),
            }
        }
        div {
            p { "Flags: " }
            input {
                value: "{start.read().flags}",
                oninput: move |e| start.write().flags = e.value.clone(),
            }
        }
        div {
            button { 
                onclick: move |_| {
                    let run = use_shared_state::<RunState>(cx).unwrap();
                    run.write().jobs.push(Job {
                        name: "Test".to_string(),
                        status: "Running".to_string(),
                        progress: 0.0,
                        eta: 0.0,
                        speed: 0.0,
                        result: "".to_string(),
                        error: "".to_string(),
                    });
                },
                "Run" 
            }
        }
        
    }
}

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

use crate::{ui::app::{run::{Job, RunState}, edit::EditState}, sim::args::Args};

pub struct StartState {
    args : Args,
}

impl Default for StartState {
    fn default() -> Self {
        Self {
            args : Args::default(),
        }
    }
}


#[inline_props]
pub(crate) fn Start(cx: Scope) -> Element {
    let start = use_shared_state::<StartState>(cx).unwrap();
    let edit = use_shared_state::<EditState>(cx).unwrap();
    render! {
        h2 {"Input"}
        div {
            p { "Ally Team: " }
            div {
                class : "form-group",
                for (i, hero) in start.read().args.allies.iter().enumerate() {
                    select {
                        id : "heroselect",
                        oninput: move |evt| {
                            println!("{evt:?}");

                            start.write().args.allies[i] = edit.read().heroes.heroes[evt.value.parse::<usize>().unwrap()].clone();
                            //.write().id = evt.value.parse::<usize>().unwrap();
                        },
                        for (i,ahero) in edit.read().heroes.heroes.iter().enumerate() {
                            option {
                                value: "{i}", 
                                selected: ahero.id == hero.id,
                                "{ahero.name}" 
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        start.write().args.allies.push(edit.read().heroes.heroes[0].clone());
                    },
                    "Add"
                }
                button {
                    onclick: move |_| {
                        start.write().args.allies.pop();
                    },
                    "Remove"
                }
            }
        }
        div {
            p { "Enemy Team: " }
            div {
                class : "form-group",
                for (i, hero) in start.read().args.enemies.iter().enumerate() {
                    select {
                        id : "heroselect",
                        oninput: move |evt| {
                            println!("{evt:?}");

                            start.write().args.enemies[i] = edit.read().heroes.heroes[evt.value.parse::<usize>().unwrap()].clone();
                            //.write().id = evt.value.parse::<usize>().unwrap();
                        },
                        for (i,ahero) in edit.read().heroes.heroes.iter().enumerate() {
                            option {
                                value: "{i}", 
                                selected: ahero.id == hero.id,
                                "{ahero.name}" 
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        start.write().args.enemies.push(edit.read().heroes.heroes[0].clone());
                    },
                    "Add"
                }
                button {
                    onclick: move |_| {
                        start.write().args.enemies.pop();
                    },
                    "Remove"
                }
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

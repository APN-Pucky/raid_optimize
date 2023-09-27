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
use std::future::IntoFuture;
use fermi::use_atom_state;


use crate::{ui::app::{run::{Job, Status,RunState}, edit::EditState}, sim::args::Args,sim::Sim};
use crate::sim::{results::CombinedResult};
//use crate::scheduler::{SCHEDULER,start_job};
//use crate::scheduler;


pub struct StartState {
    pub name : String,
    pub args : Args,
}

impl Default for StartState {
    fn default() -> Self {
        Self {
            name : "Job Name".to_string(),
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
            class : "form-group",
            label { "Job Name"}
            input {
                r#type : "text",
                value: "{start.read().name}",
                oninput: move |evt| {
                    start.write().name = evt.value.clone();
                }
            }
        }
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
            class : "form-group",
            label { "Iterations: " }
            input {
                r#type : "number",
                min : 0,
                value: "{start.read().args.iterations}",
                oninput: move |evt| {
                    if let Ok(t) = evt.value.parse::<u64>() {
                        start.write().args.iterations = t;
                    }
                }
            }
        }
        div {
            class : "form-group",
            label { "Threads: " }
            input {
                r#type : "number",
                min : 0,
                value: "{start.read().args.threads}",
                oninput: move |evt| {
                    if let Ok(t) = evt.value.parse::<u64>() {
                        start.write().args.threads = t;
                    }
                }
            }
        }
        div {
            button { 
                onclick: move |_| {
                    let run = use_shared_state::<RunState>(cx).unwrap();
                    let name = start.read().name.clone();
                    let args = start.read().args.clone();
                    let ret = cx.spawn( {
                        let run = run.to_owned();
                        async move {
                        let id = run.read().jobs.len();
                        //let name = name.to_owned();
                        //let args = args.to_owned();
                        let sim = Sim::new( args.clone());
                        run.write().jobs.push( Job  {
                            id,
                            name,
                            start_time : None,
                            args,
                            result : None,
                        });
                        let handler : tokio::task::JoinHandle<CombinedResult> =  //std::thread::spawn(move || 
                            tokio::task::spawn_blocking(move || {
                                println!("running");
                                let ret = sim.run();
                                println!("done");
                                ret
                            });
                        //)
                        let ret = handler.await.unwrap();
                        run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result = Some(ret);
                        //handler.is_finished();
                            //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result = Some(ret);
                        }
                    });

                    //let id = jobs.len();

                    //let ws : &Coroutine<JobId> =
                    //println!("test async start");
                    //cx.spawn( async move {      
                    //    let sim = Sim::new( args.clone());
                    //    let handle = std::thread::spawn(move || {
                    //        println!("test thread start");
                    //        let ret = sim.run();
                    //        println!("test thread end");
                    //        ret
                    //    });
                        //handle.join().unwrap();

                        //let _ = tokio::spawn(async move {
                        //    println!("test spawn start");
                        //    sim.run();
                        //    //tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                        //    println!("test spawn end {sim:?}");
                        //});
                        //let _ = tokio::task::spawn_local(async move {
                        //println!("test task start");
                        ////sim.run();
                        //tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                        //println!("test task end");
                        //}).await;
                    //});
                    println!("test async end");
                    /* 
                    {
                        let mut sched = use_atom_state(cx, &SCHEDULER);
                        sched.with_mut( |v| start_job(v,&scheduler::Job {
                            id:0,
                            name: start.read().name.clone(),
                            start_time : None,
                            args: start.read().args.clone(),
                            //result :None,
                        }));
                    }
                    */
                    //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result = Some(ws);
                },
                "Run" 
            }
        }
        
    }
}

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

use crate::ui::app::{ input::StartState, edit::EditState, run::RunState};
use crate::ui::app::input::Start;
//use crate::ui::app::run::Run;
use crate::ui::app::edit::Edit;

pub mod input;
pub mod run;
pub mod edit;


// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Start {},
        //#[route("/run")]
        //Run {},
        //#[route("/history")]
        //History {},
        //#[route("/edit")]
        //Edit {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}


#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            class: "navbar",
            Link { to: Route::Start {}, "Start" } 
            //Link { to: Route::Run{}, "Run" } 
            //Link { to: Route::History{}, "History" } 
            //Link { to: Route::Edit{}, "Edit" } 
        }
        Outlet::<Route> {}
    }
}




#[inline_props]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}

#[inline_props]
fn History(cx: Scope) -> Element {
    render! {
        div {
            "History"
        }
    }
}



#[inline_props]
fn Output(cx: Scope) -> Element {
    render! {
        h2 {"Output"}
        std::fs::read_dir("output/").unwrap().map(|x| {
            let path = x.unwrap().path();
            let path = path.to_str().unwrap();
            let path = path.replace("output/","");
            render!{
                button {
                    onclick : {
                        move |_| {
                            let path = format!("output/{}",path);
                            std::fs::remove_file(path).unwrap();
                        }
                    },
                    path.clone()
                }
            }
        })
    }
}


pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || {
        EditState::default() 
    });
    use_shared_state_provider(cx, || {
        RunState::default() 
    });
    use_shared_state_provider(cx, || {
        StartState::default() 
    });
    render! {
        Router::<Route> {}
    }
}
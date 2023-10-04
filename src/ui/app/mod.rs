

use dioxus::prelude::*;



//use fermi::{use_init_atom_root};




use dioxus_router::prelude::*;


use std::sync::Mutex;

use crate::ui::app::{ input::StartState, edit::EditState, run::RunState};
use crate::ui::app::input::Start;
//use crate::ui::app::run::Run;


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

pub static RUN_STATE: Mutex<RunState> = Mutex::new(RunState::new());

pub fn app(cx: Scope) -> Element {
    //use_init_atom_root(cx);
    use_shared_state_provider(cx, || {
        EditState::default() 
    });
    use_shared_state_provider(cx, || {
        &RUN_STATE
        //RunState::default() 
    });
    use_shared_state_provider(cx, || {
        StartState::default() 
    });
    render! {
        Router::<Route> {}
    }
}
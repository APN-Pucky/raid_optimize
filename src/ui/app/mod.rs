

use dioxus::prelude::*;



//use fermi::{use_init_atom_root};


use std::collections::HashMap;

use dioxus_router::prelude::*;


use std::sync::Mutex;

use crate::ui::app::{ input::StartState, edit::EditState, run::RunState};
use crate::ui::app::input::Start;

use self::run::Job;
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

#[derive(PartialEq, Props)]
pub struct JobViewProps{
    pub(crate) job_id: usize,
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

//#[inline_props]
pub fn JobView(cx: Scope<JobViewProps>) -> Element {
    use_shared_state_provider(cx, || {
        &RUN_STATE
        //RunState::default() 
    });
    let run = use_shared_state::<&Mutex<RunState>>(cx).unwrap();
    let binding = run.read().lock().unwrap();
    if let Some(job) = binding.jobs.get(cx.props.job_id) {
        render! {
            h2 { "{job.name} #{job.id}" }
            h3 { "Result" }
            div {
                div { 
                    format!("win%:\t{:>6.2} ({} / {})", job.result.wins as f64 / job.result.iterations as f64*100., job.result.wins, job.result.iterations)
                }
                div { 
                    format!("stall%:\t{:>6.2} ({} / {})", job.result.stalls as f64 / job.result.iterations as f64*100., job.result.stalls, job.result.iterations)
                }
                div { 
                    format!("loss%:\t{:>6.2} ({} / {})", job.result.losses as f64 / job.result.iterations as f64*100., job.result.losses, job.result.iterations)
                }
            }
            h3 { "Meta" }
            div {
                div { 
                    format!("Status:\t{:?}", job.status)
                }
                div { 
                    format!("Start:\t{:?}", job.start_time)
                }
                div { 
                    format!("End:\t{:?}", job.end_time)
                }
            }
            
            match job.result.statistics.get(0) {
                Some(cs) => 
                rsx!{
                    table { 
                        tr {
                            th {"Allies"},
                            job.args.allies.iter().map(|x| {
                                rsx!{
                                    th { x.name.clone() }
                                }
                            })
                        }
                        rsx! {
                        for (key,_v) in cs.hm.iter() {
                            rsx!{
                                tr {
                                    td {format!("{}",key)}
                                    for (index, _her) in job.args.allies.iter().enumerate() {
                                        rsx!{
                                            td {
                                                progress {
                                                    value : job.result.get_mean(index, key),
                                                    max : job.args.allies.iter().enumerate().map(|(index,_)| job.result.get_mean(index, key)).fold(f64::NAN, f64::max)
                                                },
                                                format!(" {:.2} +- {:.2}",job.result.get_mean(index, key), job.result.get_std(index, key))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        }
                        tr {
                            th {"Enemies"},
                            job.args.enemies.iter().map(|x| {
                                rsx!{
                                    th { x.name.clone() }
                                }
                            })
                        }
                        rsx! {
                        for (key,_v) in cs.hm.iter() {
                            rsx!{
                                tr {
                                    td {format!("{}",key)}
                                    for index in job.args.enemies.iter().enumerate().map((|(x,_)| x + job.args.allies.len())) {
                                        rsx!{
                                            td {
                                                progress {
                                                    value : job.result.get_mean(index, key),
                                                    max : job.args.enemies.iter().enumerate().map(|(index,_)| job.result.get_mean(index, key)).fold(f64::NAN, f64::max)
                                                },
                                                format!(" {:.2} +- {:.2}",job.result.get_mean(index, key), job.result.get_std(index, key))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        }
                    }
                },
                _ => rsx!{
                    "No statistics"
                }
            }
            h3 { "Arguments (Placeholder)" }
            div {
                //format!("{:#?}",job.args)
            }
            h3 { "Heroes (Placeholder)" }
            div {
                //format!("{:#?}",job.args)
            }
        }
    }
    else {
        render! {
            h2 { "Job not found" }
        }
    }
}

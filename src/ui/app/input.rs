use axum::{extract::{ws::WebSocketUpgrade, self,Path}, response::Html, routing::get, Router, Extension, Server};
use chrono::Duration;
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
use strum::IntoEnumIterator;
use chrono::{DateTime, Local};
use crate::{data::faction::Faction, ui::app::run::RUN_STATE};
use crate::data::mark::Mark;
use crate::data::class::Class;
use crate::data::rarity::Rarity;
use crate::data::skill::SkillData;
use crate::{ui::app::{run::{Job, Status,RunState}, edit::EditState}, sim::args::Args,sim::Sim};
use crate::sim::{results::CombinedResult};
use crate::wave::heroes::liz::scorched_soul::ScorchedSoul;
use std::sync::Arc;
use std::sync::Mutex;
//use crate::scheduler::{SCHEDULER,start_job};
//use crate::scheduler;
use crate::{ui::app::run, data::{heroes::Heroes, load_heroes, hero::Hero}, wave::print};
use quick_xml::se::to_string;
use tokio::sync::mpsc;
use crate::ui::login::{Login, LOGGED_IN};
static MAX_JOBS : usize = 4;
pub struct StartState {
    pub name : String,
    pub args : Args,
    pub allies : Vec<u32>,
    pub enemies : Vec<u32>,
}

impl Default for StartState {
    fn default() -> Self {
        Self {
            name : "Job Name".to_string(),
            args : Args::default(),
            allies : vec![],
            enemies : vec![],
        }
    }
}
pub fn save_to_file(heroes : &Heroes) {
    println!("Saving to file");
    let str = to_string(&heroes).unwrap();
    // write str to file
    std::fs::write("data/heroes.xml", str).unwrap();
}


macro_rules! skill_parameter_struct {
    ($edit:expr, $j:expr, $t:ty , $s:ty { $($x:expr),* }) => {
        rsx!{
            $(
            div { 
                class : "form-group",
                label {stringify!($x:)}
                input {
                    r#type : "number",
                    oninput: move |evt| {
                        let ii = $edit.read().id;
                        if let $t ($s{$x,..}) = &mut $edit.write().heroes.heroes[ii].skills[$j].data{ 
                            if let Ok(val) =  evt.value.parse::<_>() {
                                *$x =  val;
                            }
                        }
                    },
                    value : $x as f64
                }
            }
            )*
        }
    }
}

macro_rules! skill_parameter {
    ($edit:expr, $j:expr, $s:ty { $($x:expr),* }) => {
        rsx!{
            $(
            div { 
                class : "form-group",
                label {stringify!($x:)}
                input {
                    r#type : "number",
                    oninput: move |evt| {
                        let ii = $edit.read().id;
                        if let $s{$x,..} = &mut $edit.write().heroes.heroes[ii].skills[$j].data{ 
                            if let Ok(val) =  evt.value.parse::<_>() {
                                *$x =  val;
                            }
                        }
                    },
                    value : $x as f64
                }
            }
            )*
        }
    }
}

macro_rules! hero_positive_number {
    ($edit:expr, $prop:expr) => {
        rsx!{
            div {
                class : "form-group",
                label {r#for : stringify!($prop), 
                    stringify!($prop:)
                },
                input {
                    id : stringify!($prop),
                    r#type : "number",
                    min : 0,
                    value : $edit.read().heroes.heroes[$edit.read().id].$prop as f64,
                    //value : "{$edit.read().heroes.heroes[$edit.read().id].health}",
                    //value : concat!("{",stringify!($value),"}"),
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = $edit.read().id;
                            $edit.write().heroes.heroes[ii].$prop= i;
                        };
                    },
                }
            }
        }
    }
}

#[inline_props]
pub(crate) fn Start(cx: Scope) -> Element {
    let start = use_shared_state::<StartState>(cx).unwrap();
    let edit = use_shared_state::<EditState>(cx).unwrap();
    let run = use_shared_state::<&Mutex<RunState>>(cx).unwrap();
    //let run = use_atom_ref(&cx,RUN_STATE);
    //let run = use_atom_state(cx, &RUN_STATE);
    if edit.read().auto_safe {
        save_to_file(&edit.read().heroes);
    }
    render! {
        div { 
            class : "container2",
      div {
        class : "resize",
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
                for (i, heroid) in start.read().allies.iter().enumerate() {
                    select {
                        id : "heroselect",
                        oninput: move |evt| {
                            println!("{evt:?}");

                            start.write().allies[i] = evt.value.parse::<u32>().unwrap();
                            //.write().id = evt.value.parse::<usize>().unwrap();
                        },
                        for (i,ahero) in edit.read().heroes.heroes.iter().enumerate() {
                            option {
                                value: "{ahero.id}", 
                                selected: ahero.id == *heroid,
                                "{ahero.name}" 
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        start.write().allies.push(edit.read().heroes.heroes[0].id);
                    },
                    "Add"
                }
                button {
                    onclick: move |_| {
                        start.write().allies.pop();
                    },
                    "Remove"
                }
            }
        }
        div {
            p { "Enemy Team: " }
            div {
                class : "form-group",
                for (i, heroid) in start.read().enemies.iter().enumerate() {
                    select {
                        id : "heroselect",
                        oninput: move |evt| {
                            println!("{evt:?}");

                            start.write().enemies[i] = evt.value.parse::<u32>().unwrap();
                            //.write().id = evt.value.parse::<usize>().unwrap();
                        },
                        for (i,ahero) in edit.read().heroes.heroes.iter().enumerate() {
                            option {
                                value: "{ahero.id}", 
                                selected: ahero.id == *heroid,
                                "{ahero.name}" 
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        start.write().enemies.push(edit.read().heroes.heroes[0].id);
                    },
                    "Add"
                }
                button {
                    onclick: move |_| {
                        start.write().enemies.pop();
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
        if LOGGED_IN.lock().unwrap().login == Login::None {
            rsx!{
                div {
                class : "form-group",
                label { "Threads (Log in to unlock): " }
                    input { 
                        readonly: true,
                        disabled: true,
                        r#type : "number",
                        min : 1,
                        max : 1,
                        value : 1
                    }
                }
            }
        }
        else {
            rsx!{
                div {
                    class : "form-group",
                    label { "Threads: " }
                    input {
                        r#type : "number",
                        min : 1,
                        value: "{start.read().args.threads}", 
                        oninput: move |evt| {
                            if let Ok(t) = evt.value.parse::<u64>() {
                                start.write().args.threads = t;
                            }
                        }
                    }
                }
            }
        }
        div {
            button { 
                onclick : move |_| {
                    if LOGGED_IN.lock().unwrap().login == Login::None  && run.read().lock().unwrap().jobs.iter().filter(|j| j.status == Status::Running).count() >= MAX_JOBS {
                        return;
                    }
                    {
                        // set allies and enemies in args
                        let al = start.read().allies.iter().map(|i| edit.read().heroes.heroes.iter().find(|&h| h.id == *i).unwrap().clone()).collect();
                        start.write().args.allies = al;
                        let el = start.read().enemies.iter().map(|i| edit.read().heroes.heroes.iter().find(|&h| h.id == *i).unwrap().clone()).collect();
                        start.write().args.enemies = el;
                    }
                    let name = start.read().name.clone();
                    let args = start.read().args.clone();
                    let ret = cx.spawn( {
                        let run = run.to_owned();
                        async move {
                        let id = run.read().lock().unwrap().jobs.len();
                        //let id = run.read().jobs.len();
                        let sim = Sim::new( args.clone());
                        //run.write().jobs.push( Job  {
                        run.write().lock().unwrap().jobs.push( Job  {
                            id,
                            name,
                            status : Status::Running,
                            start_time : Some(Local::now()),
                            end_time : None,
                            args,
                            result : CombinedResult::default(),
                        });
                        let (tx, mut rx) = mpsc::unbounded_channel::<CombinedResult>();
                        let handler  =  //std::thread::spawn(move || 
                            tokio::task::spawn_blocking(move || {
                                println!("running");
                                let ret = sim.run(tx);
                                println!("done");
                                //ret
                            });

                        println!("awaiting");
                        while let Some(cr) = rx.recv().await {
                            //println!("received");
                            //CombinedResult::add_combined_result(&mut run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result, &cr)
                            CombinedResult::add_combined_result(&mut run.write().lock().unwrap().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result, &cr)
                            //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result.add_combined_result(cr);
                        }
                        //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().end_time= Some(Local::now());
                        //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().status= Status::Ended;
                        run.write().lock().unwrap().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().end_time= Some(Local::now());
                        run.write().lock().unwrap().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().status= Status::Ended;
                        //)
                        //if let Ok(ret) = handler.await {
                        //    println!("writing");
                        //    run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().end_time= Some(Local::now());
                        //    run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result = Some(ret);
                        //    run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().status= Status::Ended;
                        //}
                        //else {
                        //    run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().status= Status::Failed;
                        //}
                        //handler.is_finished();
                            //run.write().jobs.iter_mut().filter(|j| j.id == id).nth(0).unwrap().result = Some(ret);
                        }
                    });
                },
                if LOGGED_IN.lock().unwrap().login == Login::None {
                    format!("Run {}/{} (Log in to unlock)",run.read().lock().unwrap().jobs.iter().filter(|j| j.status == Status::Running).count()+1, MAX_JOBS)
                }
                else {
                    "Run".to_string()
                }
            }
        }
        
    }
    div {
        class : "resize",
        div {
            class: "container",
            div {
                class : "column files" ,
                label { 
                    r#for : "heroselect",
                    "Heroes" }
                select {
                    id : "heroselect",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        edit.write().id = evt.value.parse::<usize>().unwrap();
                    },
                    for (i,hero) in edit.read().heroes.heroes.iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected: i == edit.read().id,
                            "{hero.name}" 
                        }
                    }
                }
            }
            div {
                class : "column properties" ,
                button {
                    onclick: move |_| {
                        let mut hero = Hero::default();
                        let mut max = 0;
                        for h in edit.read().heroes.heroes.iter() {
                            if h.id > max {
                                max = h.id;
                            }
                        }
                        hero.id = max + 1;
                        edit.write().heroes.heroes.push(hero);
                        let id = edit.read().heroes.heroes.len() - 1;
                        edit.write().id = id;
                    },
                    "New"
                }
            }
            div {
                class : "column properties" ,
                button {
                    onclick: move |_| {
                        let i = edit.read().id;
                        let mut hero = edit.read().heroes.heroes[i].clone();
                        hero.id = edit.read().heroes.heroes.len() as u32;
                        edit.write().heroes.heroes.push(hero);
                        let id = edit.read().heroes.heroes.len() - 1;
                        edit.write().id = id;
                    },
                    "Clone"
                }
            }
            div {
                class : "column inputs " ,
                button {
                    onclick: move |_| {
                        let ii = edit.read().id;
                        edit.write().heroes.heroes.remove(ii);
                        edit.write().id = std::cmp::max(ii-1,0);
                    },
                    "Delete"
                }
            }
            div {
                class : "column inputs " ,
                button {
                    onclick: move |_| {
                        edit.write().heroes = load_heroes("data/heroes.xml".to_string());
                    },
                    "Reload"
                }
            }
            div {
                class : "column inputs " ,
                
                div {
                    class : "form-group",
                    button {
                        onclick: move |_| {
                            save_to_file(&edit.read().heroes);
                        },
                        "Save"
                    }
                    input {
                        id : "auto_safe",
                        value: "auto_safe",
                        r#type : "checkbox",
                        checked : "{edit.read().auto_safe}",
                        onchange: move |e| {
                            edit.write().auto_safe = e.value.parse::<bool>().unwrap();
                        },
                    }
                    label {
                        r#for : "auto_safe",
                        "auto safe"
                    }
                }
            }
        }
        div {
            class: "container scrollable-div",
        div {
            div {
                class : "form-group",
                label {r#for : "id", "id:"},
                input {
                    id : "id",
                    r#type : "number",
                    min : 0,
                    value: "{edit.read().heroes.heroes[edit.read().id].id}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<u32>() {
                            let ii = edit.read().id;
                            edit.write().heroes.heroes[ii].id = i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "name", "name:"},
                input {
                    id : "name",
                    value: "{edit.read().heroes.heroes[edit.read().id].name}",
                    oninput: move |e| {
                        let ii = edit.read().id;
                        edit.write().heroes.heroes[ii].name = e.value.clone();
                    },
                }
            }
            hero_positive_number!(edit,health)
            hero_positive_number!(edit,attack)
            hero_positive_number!(edit,defense)
            hero_positive_number!(edit,speed)
            hero_positive_number!(edit,crit_rate)
            hero_positive_number!(edit,crit_damage)
            hero_positive_number!(edit,effect_hit)
            hero_positive_number!(edit,effect_resistance)
            hero_positive_number!(edit,mastery)
            hero_positive_number!(edit,healing_effect)
            hero_positive_number!(edit,leech)
            hero_positive_number!(edit,piercing)
            hero_positive_number!(edit,tenacity)
            hero_positive_number!(edit,damage_reflection)
            div {
                class : "form-group",
                label {r#for : "mark", "mark"},
                select {
                    id : "mark",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        let ii = edit.read().id;
                        edit.write().heroes.heroes[ii].mark = *Mark::iter().collect::<Vec<Mark>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,mark) in Mark::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : edit.read().heroes.heroes[edit.read().id].mark == mark,
                            "{mark}"
                        }
                    }
                }
            }
            div {
                class : "form-group",
                label {r#for : "class", "class"},
                select {
                    id : "class",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        let ii = edit.read().id;
                        edit.write().heroes.heroes[ii].class = *Class::iter().collect::<Vec<Class>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,class) in Class::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : edit.read().heroes.heroes[edit.read().id].class == class,
                            "{class}"
                        }
                    }
                }
            }
            div {
                class : "form-group",
                label {r#for : "faction", "faction"},
                select {
                    id : "faction",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        let ii = edit.read().id;
                        edit.write().heroes.heroes[ii].faction= *Faction::iter().collect::<Vec<Faction>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,faction) in Faction::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : edit.read().heroes.heroes[edit.read().id].faction== faction,
                            "{faction}"
                        }
                    }
                }
            }
            div {
                class : "form-group",
                label {r#for : "rarity", "rarity"},
                select {
                    id : "rarity",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        let ii = edit.read().id;
                        edit.write().heroes.heroes[ii].rarity = *Rarity::iter().collect::<Vec<Rarity>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,rarity) in Rarity::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : edit.read().heroes.heroes[edit.read().id].rarity== rarity,
                            "{rarity}"
                        }
                    }
                }
            }
        }
        div {
            h3{"Skills"}
            for (j,s) in edit.read().heroes.heroes[edit.read().id].skills.iter().enumerate() {
                
                div {
                    class : "form-group",
                    select {
                        oninput: move |evt| {
                            println!("{evt:?}");
                            let ii = edit.read().id;
                            edit.write().heroes.heroes[ii].skills[j].data = SkillData::iter().collect::<Vec<SkillData>>().get(evt.value.parse::<usize>().unwrap()).unwrap().clone();
                        },
                        for (i,sd) in SkillData::iter().enumerate() {
                            option {
                                value: "{i}", 
                                selected : format!("{}",edit.read().heroes.heroes[edit.read().id].skills[j].data) == format!("{}",sd),
                                "{sd}"
                            }
                        }
                        //class : "full-input",
                        //value : "{s.data}"
                    },
                }
                div {
                    class : "form-group",
                    label {"Cooldown: "}
                    input {
                        value : "{s.cooldown}"
                    }
                }
                match edit.read().heroes.heroes[edit.read().id].skills[j].data {
                        //SkillData::BasicAttack {attack_damage_ratio} => 
                        //    skill_parameter!(edit,j, SkillData::BasicAttack {attack_damage_ratio}),

                        SkillData::ScorchedSoul(ScorchedSoul{ attack_damage_ratio , hp_burning_chance, hp_burning_turns }) => 
                            skill_parameter_struct!(edit,j, SkillData::ScorchedSoul , ScorchedSoul{attack_damage_ratio , hp_burning_chance, hp_burning_turns}),
                        SkillData::FireHeal{ heal_attack_ratio, heal_max_hp_ratio, block_debuff_turns} =>
                            skill_parameter!(edit,j, SkillData::FireHeal {heal_attack_ratio, heal_max_hp_ratio, block_debuff_turns}),
                        SkillData::Resurrection{ shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio} =>
                            skill_parameter!(edit,j, SkillData::Resurrection {shield_max_hp_ratio, shield_turns, cleanse_dot_debuffs, restore_max_hp_ratio}),

                        SkillData::ScytheStrike{ attack_damage_ratio, bleed_chance,bleed_turns} =>
                            skill_parameter!(edit,j, SkillData::ScytheStrike {attack_damage_ratio, bleed_chance,bleed_turns}),
                        SkillData::BloodthirstyScythe{ attack_damage_ratio, bleed_chance,bleed_turns} => 
                            skill_parameter!(edit,j, SkillData::BloodthirstyScythe {attack_damage_ratio, bleed_chance,bleed_turns}),
                        SkillData::EnergyBurst{attack_damage_ratio, bleed_turns, reduce_effect_resistance_chance, reduce_effect_resistance_turns} =>
                            skill_parameter!(edit,j, SkillData::EnergyBurst {attack_damage_ratio, bleed_turns, reduce_effect_resistance_chance, reduce_effect_resistance_turns}),
                        _ => {
                            rsx! {
                                div {
                                    "No inputs (yet)"
                                }
                            }
                        }
                        
                }
            }
        }
    }
    }
}
        h2 {"Jobs"}
        div {
            // table of jobs gg
            table {
                tr {
                    th { "ID" }
                    th { "Name" }
                    th { "Start" }
                    th { "Progress" }
                    th { "ETA/Time" }
                    th { "Status" }
                    th { "Wins" }
                    th { "Stalls" }
                    th { "Losses" }
                }
                for job in run.read().lock().unwrap().jobs.iter().rev() {
                            rsx! {
                                tr {
                                    td { "{job.id}" }
                                    td { "{job.name}" }
                                    match job.start_time {
                                        Some(end) => rsx!{
                                            td { "{end.to_rfc2822()}" }},
                                        None => rsx!{ td {""}}
                                    }
                                    td { 
                                        progress {
                                            value : "{job.result.wins + job.result.stalls + job.result.losses}",
                                            max : "{job.args.iterations}"
                                        }
                                        
                                    }
                                    if let Some(end) = job.end_time {
                                            let duration : Duration = end - job.start_time.unwrap();
                                            let days = duration.num_seconds() / (60 * 60 * 24);
                                            let hours = (duration.num_seconds() / (60 * 60)) % 24;
                                            let minutes = (duration.num_seconds() / 60) % 60;
                                            let seconds = duration.num_seconds() % 60;
                                            rsx!{td{ format!("{}d {}h {}m {}s", days, hours, minutes,seconds)}}
                                        }
                                        else {
                                            // compute ETA
                                            if job.result.wins + job.result.stalls + job.result.losses == 0 {
                                                rsx!{ td {"?"}}
                                            }
                                            else {
                                                let mut duration :Duration = Local::now().signed_duration_since(job.start_time.unwrap());
                                                duration = duration /((job.result.wins + job.result.stalls + job.result.losses) as i32) * ((job.args.iterations - (job.result.wins + job.result.stalls + job.result.losses)) as i32);
                                                let days = duration.num_seconds() / (60 * 60 * 24);
                                                let hours = (duration.num_seconds() / (60 * 60)) % 24;
                                                let minutes = (duration.num_seconds() / 60) % 60;
                                                let seconds = duration.num_seconds() % 60;
                                                rsx!{td { format!("{}d {}h {}m {}s", days, hours, minutes,seconds) }}
                                            }
                                        }
                                    //match job.end_time {
                                    //    Some(end) => rsx!{
                                    //        td { "{end.to_rfc2822()}" }},
                                    //    None =>  
                                    //            rsx!{td {""}}
                                    //}
                                    td { "{job.status}" }
                                    td { "{job.result.wins}" }
                                    td { "{job.result.stalls}" }
                                    td { "{job.result.losses}" }
                                }
                            }
                }
            }
        }
    }

}

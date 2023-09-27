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
use crate::{ui::app::run::{Job, RunState}, data::{heroes::Heroes, load_heroes, hero::Hero}, wave::print};
use quick_xml::se::to_string;
use crate::data::faction::Faction;
use crate::data::mark::Mark;
use crate::data::class::Class;
use crate::data::rarity::Rarity;
use strum::IntoEnumIterator;

pub struct EditState {
    pub heroes : Heroes,
    pub id: usize,
    pub auto_safe : bool
}

impl Default for EditState {
    fn default() -> Self {
        let heroes = load_heroes("data/heroes.xml".to_string());
        Self {
            heroes,
            id: 0,
            auto_safe : false,
        }
    }
}

pub fn save_to_file(heroes : &Heroes) {
    println!("Saving to file");
    let str = to_string(&heroes).unwrap();
    // write str to file
    std::fs::write("data/heroes.xml", str).unwrap();
}

#[inline_props]
pub(crate) fn Edit(cx: Scope) -> Element {
    let heroes = use_shared_state::<EditState>(cx).unwrap();
    if heroes.read().auto_safe {
        save_to_file(&heroes.read().heroes);
    }
    render! {
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
                        heroes.write().id = evt.value.parse::<usize>().unwrap();
                    },
                    for (i,hero) in heroes.read().heroes.heroes.iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected: i == heroes.read().id,
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
                        hero.id = heroes.read().heroes.heroes.len() as u32;
                        heroes.write().heroes.heroes.push(hero);
                        let id = heroes.read().heroes.heroes.len() - 1;
                        heroes.write().id = id;
                    },
                    "New"
                }
            }
            div {
                class : "column properties" ,
                button {
                    onclick: move |_| {
                        let i = heroes.read().id;
                        let mut hero = heroes.read().heroes.heroes[i].clone();
                        hero.id = heroes.read().heroes.heroes.len() as u32;
                        heroes.write().heroes.heroes.push(hero);
                        let id = heroes.read().heroes.heroes.len() - 1;
                        heroes.write().id = id;
                    },
                    "Clone"
                }
            }
            div {
                class : "column inputs " ,
                button {
                    onclick: move |_| {
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes.remove(ii);
                        heroes.write().id = std::cmp::max(ii-1,0);
                    },
                    "Delete"
                }
            }
            div {
                class : "column inputs " ,
                button {
                    onclick: move |_| {
                        heroes.write().heroes = load_heroes("data/heroes.xml".to_string());
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
                            save_to_file(&heroes.read().heroes);
                        },
                        "save"
                    }
                    input {
                        id : "auto_safe",
                        value: "auto_safe",
                        r#type : "checkbox",
                        checked : "{heroes.read().auto_safe}",
                        onchange: move |e| {
                            heroes.write().auto_safe = e.value.parse::<bool>().unwrap();
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
            class: "container",
        div {
            div {
                class : "form-group",
                label {r#for : "id", "id:"},
                input {
                    id : "id",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].id}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<u32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].id = i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "name", "name:"},
                input {
                    id : "name",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].name}",
                    oninput: move |e| {
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes[ii].name = e.value.clone();
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "health", "health:"},
                input {
                    id : "health",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].health}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].health = i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "attack", "attack:"},
                input {
                    id : "attack",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].attack}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].attack= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "defense", "defense:"},
                input {
                    id : "defense",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].defense}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].defense= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "speed", "speed:"},
                input {
                    id : "speed",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].speed}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].speed= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "crit_rate", "crit_rate:"},
                input {
                    id : "crit_rate",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].crit_rate}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].crit_rate= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "crit_damage", "crit_damage:"},
                input {
                    id : "crit_rate",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].crit_damage}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].crit_damage= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "effect_hit", "effect_hit:"},
                input {
                    id : "effect_hit",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].effect_hit}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].effect_hit= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "effect_resistance", "effect_resistance:"},
                input {
                    id : "effect_resistance",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].effect_resistance}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].effect_resistance= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "mastery", "mastery"},
                input {
                    id : "mastery",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].mastery}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].mastery= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "healing_effect", "healing_effect"},
                input {
                    id : "healing_effect",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].healing_effect}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].healing_effect= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "leech", "leech"},
                input {
                    id : "leech",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].leech}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].leech= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "piercing", "piercing"},
                input {
                    id : "piercing",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].piercing}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].piercing= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "tenacity", "tenacity"},
                input {
                    id : "tenacity",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].tenacity}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].tenacity= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "damage_reflection", "damage_reflection"},
                input {
                    id : "damage_reflection",
                    value: "{heroes.read().heroes.heroes[heroes.read().id].damage_reflection}",
                    oninput: move |e| {
                        if let Ok(i) = e.value.parse::<f32>() {
                            let ii = heroes.read().id;
                            heroes.write().heroes.heroes[ii].damage_reflection= i;
                        };
                    },
                }
            }
            div {
                class : "form-group",
                label {r#for : "mark", "mark"},
                select {
                    id : "mark",
                    oninput: move |evt| {
                        println!("{evt:?}");
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes[ii].mark = *Mark::iter().collect::<Vec<Mark>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,mark) in Mark::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : heroes.read().heroes.heroes[heroes.read().id].mark == mark,
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
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes[ii].class = *Class::iter().collect::<Vec<Class>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,class) in Class::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : heroes.read().heroes.heroes[heroes.read().id].class == class,
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
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes[ii].faction= *Faction::iter().collect::<Vec<Faction>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,faction) in Faction::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : heroes.read().heroes.heroes[heroes.read().id].faction== faction,
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
                        let ii = heroes.read().id;
                        heroes.write().heroes.heroes[ii].rarity = *Rarity::iter().collect::<Vec<Rarity>>().get(evt.value.parse::<usize>().unwrap()).unwrap();
                    },
                    for (i,rarity) in Rarity::iter().enumerate() {
                        option {
                            value: "{i}", 
                            selected : heroes.read().heroes.heroes[heroes.read().id].rarity== rarity,
                            "{rarity}"
                        }
                    }
                }
            }
        }
        div  {

        }
        }
    }
}
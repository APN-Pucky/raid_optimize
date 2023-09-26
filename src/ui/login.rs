use axum::{extract::{ws::WebSocketUpgrade, self,Path}, response::Html, routing::get, Router, Extension, Server};
use axum::http::Uri;
use axum::extract::Host;
use axum::body::Body;
use axum::http::Request;
use serde_json::json;
use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::{Error, Client, Response};
use dioxus_router::prelude::*;
use std::collections::HashMap;

use crate::ui::user::Login;

use super::user::UserData;

#[derive(Deserialize, Serialize, Debug, )]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
    refresh_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Subscription {
    active: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User{
    name:String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Subscriber {
    subscritption: Subscription,

}



#[derive(Deserialize, Serialize, Debug, Clone)]
struct Data {
    user: User,
//    subscriber: Subscriber,
}
#[derive(Deserialize, Serialize, Debug)]
struct Query {
    data: Data,
//    subscriber: Subscriber,
}

const SCHEMA: &str = r#"
{ user { name } }
"#;


pub async fn check_login(uri :&Uri) -> UserData {
    println!("Received parameter: {:?}", uri);
    // extract ?code=XYZ from uri
    

    let uri = format!("http://localhost/{}",uri);
    let url = Url::parse(&uri).unwrap();
    let code = url.query_pairs()
    .find(|(key, _)| key == "code")
    .map(|(_, value)| value.to_string());


    if let Some(code) = code {
        println!("The code is {}", code); // The code is XYZ
            // POST to https://www.subscribestar.com/api/oauth2/token
            // with body: grant_type=authorization_code&code=XYZ&redirect_uri=http%3A%2F%2Flocalhost%3A3030
            let request_url = format!("https://www.subscribestar.com/oauth2/token?client_id={YOUR_CLIENT_ID}&client_secret={YOUR_CLIENT_SECRET}&code={RETURNED_CODE}&grant_type=authorization_code&redirect_uri={YOUR_REDIRECT_URL}",
                RETURNED_CODE= code,
                YOUR_CLIENT_SECRET = "yjfi3s5pULh6RqgnzSKCQEz-m0bVXLQsKdhyXyVyKYs",
                YOUR_CLIENT_ID = "7ri7YUceDuUo2btnbf0XfINAHbqomSxE4oaHcBcvxKA",
                YOUR_REDIRECT_URL = "http%3A%2F%2Flocalhost%3A3030",
                    );
        println!("{}", request_url);
        let response :Response = Client::new().post(&request_url).send().await.unwrap();
        //let response  = reqwest::get(&request_url).await.unwrap();
        println!("{:?}", response);
        // JSON parse reply to User
        match response.json().await {
            Ok(Token{ access_token, token_type, expires_in, refresh_token }) 
                =>  {
                    //println!("User: {:?}", User{ access_token, token_type, expires_in, refresh_token });
                    println!("ok");
                    let s = format!("{} {}",token_type,access_token);
                    println!("header {}",s);
                    let response :Response = Client::new()
                        .post("https://www.subscribestar.com/api/graphql/v1")
                        .header("Authorization", s)
                        .json(&json!({
                            "query": SCHEMA,
                        }))
                        .send().await.unwrap();
                    // construct graphql querry from Data struct
                    let ret = response.text().await;
                    println!("{:?}", ret);
                    if let Ok(good) = ret {
                        match serde_json::from_str::<Query>(&good){
                            Ok(Query {data }) =>  {
                                println!("User: {:?}", data.user);
                                //println!("Subscriber: {:?}", data.subscriber);
                                
                                return UserData {
                                    name : data.user.name,
                                    login : Login::SubscribeStar,
                                };
                            }
                            Err(e) => {
                                println!("Error Parse {:?}",e);
                            }
                        }
                    }
                    else {
                        println!("Error token");
                    }
                    //println!("{:?}", response.text().await.unwrap());
                    //nav = format!("{:?}",queryd.data.user.name);
                }
            _ => println!("Error"),
        }
    } else {
        println!("No code provided");
    }
    return UserData{login : Login::None , name : "".to_string()};
}
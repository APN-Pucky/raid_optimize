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
use axum_server::HttpConfig;

use crate::ui::{login::check_login, user::{UserData, Login}};

pub mod login;
pub mod user;

pub mod app;



pub async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move |Host(hostname): Host,request: Request<Body>| async move {
                // get code from url
                //let code = extract::FromRequest::from_request(.await.unwrap();
                //println!("Hostname: {}", hostname);
                let user:UserData =check_login(request.uri()).await;
                let mut nav :String = "".to_string();

                match user.login  {
                    Login::None => {
                nav=r#"
                <a href="https://www.subscribestar.com/oauth2/authorize?client_id=7ri7YUceDuUo2btnbf0XfINAHbqomSxE4oaHcBcvxKA&redirect_uri=http%3A%2F%2Flocalhost%3A3030&response_type=code&scope=content_provider_profile.read+content_provider_profile.subscriptions.read+content_provider_profile.payments.read+content_provider_profile.payouts.read+subscriber.read+subscriber.payments.read+user.read+user.email.read+user.shipping_address.read+user.subscriptions.read+user.payments.read" class="tablinks">Login</a>
                <a href="https://subscribestar.com/apn-pucky">Unlock all features for 10$ per month: https://subscribestar.com/apn-pucky</a>
                "#.to_string();
                    }
                    Login::SubscribeStar => {
                        nav = format!("<a>Welcome {} <a/>",user.name);
                    }
                }

                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> 
                    <title>Raid Optimize</title>  
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <style>

                        body {{
                            font-family: Arial, sans-serif;
                            margin: 0;
                        }}
                
                        .navbar {{
                            background-color: #333;
                            overflow: hidden;
                        }}

                        #loginbar {{
                            background-color: #444;
                            overflow: hidden;
                        }}
                
                        .navbar a {{
                            float: left;
                            display: block;
                            color: #f2f2f2;
                            text-align: center;
                            padding: 14px 16px;
                            text-decoration: none;
                        }}
                
                        .navbar a:hover {{
                            background-color: #ddd;
                            color: black;
                        }}
                
                        .tabcontent {{
                            display: none;
                        }}
                
                        .tabcontent.active {{
                            display: block;
                        }}
                        /* Table style */
                        table {{
                          border-collapse: collapse;
                          width: 100%;
                        }}

                        /* Table header style */
                        th, td {{
                          border: 1px solid #dddddd;
                          text-align: left;
                          padding: 8px;
                        }}

                        /* Alternate row color */
                        tr:nth-child(even) {{
                          background-color: #f2f2f2;
                        }}

                        /* Hover effect */
                        tr:hover {{
                          background-color: #ddd;
                        }}

                          .container {{
                            display: flex;
                          }}
                        
                          .column {{
                            flex: 1;
                            padding: 10px;
                          }}
                        
                          .files {{
                            background-color: #f2f2f2;
                          }}
                        
                          .properties {{
                            background-color: #e0e0e0;
                          }}
                        
                          .inputs {{
                            background-color: #d8d8d8;
                          }}
                        
                          select, input {{
                            width: 100%;
                            padding: 5px;
                            margin: 5px 0;
                          }}
                          .form-group {{
                            margin-bottom: 15px;
                            display: flex;
                            align-items: center;
                          }}
                          label {{
                              display: inline-block;
                              width: 200px;
                          }}
                          input {{
                              display: inline-block;
                              width: 100px;
                          }}
                    </style>
                </head>
                <body> 
                <div class="navbar" id="loginbar">
                        {nav}
                </div>
                <div id="main"></div> 
                </body>
                {glue}
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app::app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    //let path = "http://127.0.0.1:3030";

    //match open::that(path) {
    //    Ok(()) => println!("Opened '{}' successfully.", path),
    //    Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    //}

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


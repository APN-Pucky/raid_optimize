use axum::{extract::{ws::WebSocketUpgrade, Path}, response::Html, routing::get, Router};
use std::collections::HashMap;

use dioxus::prelude::*;
use axum::extract::Host;
use axum::body::Body;
use axum::http::Request;








use crate::ui::{login::{check_login, UserData, Login}, app::JobViewProps};

pub mod login;

pub mod app;



pub async fn main() {
  // read css from data/layout.css file
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move |Host(_hostname): Host,request: Request<Body>| async move {
              let css = std::fs::read_to_string("data/layout.css").unwrap();
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
                        nav = format!("<a>Welcome {} <a/>",user.name.unwrap());
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
                    {css}
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
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws/-1"))
                ))
            }),
        )
        .route("/job/:job_id", get(move |Path(params) : Path<HashMap<String,String>>| async move {
              let css = std::fs::read_to_string("data/layout.css").unwrap();
            Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> 
                    <title>Raid Optimize</title>  
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <style>
                      {css}
                    </style>
                </head>
                <body> 
                <div id="main"></div> 
                </body>
                {glue}
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws/{}",params.get("job_id").unwrap()))
                ))
        }))
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws/:job_id",
            get(move |ws: WebSocketUpgrade, Path(params) : Path<HashMap<String,String>>| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    let j = params.get("job_id").unwrap();
                    if j == "-1" {
                        _ = view.launch(dioxus_liveview::axum_socket(socket), app::app).await;
                    }
                    else {
                        _ = view.launch_with_props(dioxus_liveview::axum_socket(socket), app::JobView, JobViewProps {job_id : j.parse::<usize>().unwrap()}).await;
                    }
                })
            }),
        );

    println!("Listening on http://{addr}");

    let path = "http://127.0.0.1:3030";
    match open::that(path) {
        Ok(()) => println!("Opened '{}' successfully.", path),
        Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
    }

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


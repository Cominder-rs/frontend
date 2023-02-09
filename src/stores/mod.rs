use std::str::FromStr;
use gloo_net::http::{ReferrerPolicy, Request};
use serde::Serialize;
use serde_wasm_bindgen::to_value;

use tonic_web_wasm_client::Client;
use helloworld::users_client::UsersClient;
use helloworld::HelloRequest;

use time::OffsetDateTime;

use gloo_console::log;

pub mod helloworld {
    tonic::include_proto!("users");
}

pub struct Store;

#[derive(Serialize)]
struct LoginData {
    pub login: String,
    pub password: String,
}

impl Store {
    pub fn login(login: String, password: String) {
        let start = OffsetDateTime::now_utc();

        let base_url = "http://127.0.0.1:50051";
        let mut client = UsersClient::new(Client::new(base_url.to_string()));
        wasm_bindgen_futures::spawn_local(async move {
                //
                // gloo_net::http::Request::get(base_url)
                //     .query([("message", "Tonic")])
                //     .send()
                //     .await
                //     .unwrap();

                let request = tonic::Request::new(HelloRequest {
                    name: "Tonic".to_string()
                });

                client.say_hello(request).await.unwrap();
                //

            });
    }
}

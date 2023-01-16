use gloo_net::http::{ReferrerPolicy, Request};
use serde::Serialize;
use serde_wasm_bindgen::{to_value};

pub struct Store;

#[derive(Serialize)]
struct LoginData {
    pub login: String,
    pub password: String,
}

impl Store {
    pub fn login(login: String, password: String) {
        wasm_bindgen_futures::spawn_local(async move {
            let data = LoginData {
                login,
                password
            };

            // let data = to_value(&data).unwrap();

            let send_req = Request::post("http://127.0.0.1:8080/login")
                .json(&data)
                .unwrap()
                .send()
                .await
                .unwrap();
        });
    }
}
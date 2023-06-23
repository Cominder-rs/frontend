use core::fmt::Debug;
use std::fmt::Display;
use js_sys::RegExp;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use tonic::{Code, Status};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{
    core::types::Response,
    stores::notifier_store::{NotificationVariant, NotifierStore},
};

pub fn get_element_by_id<T>(id: &str) -> T
where
    T: JsCast,
{
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into::<T>()
}

pub fn is_err<I, O>(action: Action<I, Response<O>>) -> bool {
    action.value().with(|value| {
        if let Some(response) = value {
            response.is_err()
        } else {
            false
        }
    })
}

pub fn handle_response<T: Debug>(cx: Scope, response: Response<T>) -> Response<T> {
    let mut notiifer_store = use_context::<NotifierStore>(cx).unwrap();
    if let Err(ref err) = response {
        cfg_if::cfg_if! {
            if #[cfg(any(feature = "dev", feature = "test"))] {
                debug_warn!("{}", &err);
            }
        }
        match err.code() {
            Code::Unauthenticated => {
                _ = use_navigate(cx)("/auth", NavigateOptions::default());
                notiifer_store.notify(
                    cx,
                    NotificationVariant::Info,
                    "Необходима авторизация".to_string(),
                    None,
                );
                let storage = window().local_storage().unwrap().unwrap();
                _ = storage.remove_item("token");
            }
            Code::Internal | Code::Unknown => {
                notiifer_store.notify(
                    cx,
                    NotificationVariant::Error,
                    "Произошла ошибка".to_string(),
                    None,
                );
            }
            _ => (),
        }
        return response
    }
    response 
}

pub fn match_media(query: &str) -> bool {
    window()
        .match_media(query)
        .is_ok_and(|mql| mql.map_or(false, |mql| mql.matches()))
}

pub fn input_mask(signal: RwSignal<String>, event: ev::Event, regex: &str, flags: Option<&str>) {
    let current_value = signal.get_untracked();
    let entered_value = event_target_value(&event);
    let regex = RegExp::new(regex, flags.unwrap_or(""));
    if regex.test(&entered_value) {
        signal.set(entered_value);
    } else {
        event_target::<HtmlInputElement>(&event).set_value(&current_value);
    }
}

pub fn map_status_and_err<I, S>(action: Action<I, Option<Status>>, err: S) -> bool 
where
    S: From<String> + Eq + PartialEq + Display
{
    if let Some(Some(st)) = action.value()() {
        let message = st.message().to_owned();
        let error = S::from(message);
        error == err
    } else {
        false
    }
}

pub fn map_status_and_err2<I, S>(action: Action<I, Option<Status>>, err: S, signal: RwSignal<bool>) -> RwSignal<bool>
where
    S: From<String> + Eq + PartialEq + Display
{
    if let Some(Some(st)) = action.value()() {
        let message = st.message().to_owned();
        let error = S::from(message);
        if error == err {
            signal.set(true)
        }
        signal
    } else {
        signal.set(false);
        signal
    }
}
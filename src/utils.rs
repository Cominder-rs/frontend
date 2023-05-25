use wasm_bindgen::JsCast;
use leptos::*;
use std::any::Any;
use crate::core::types::Response;
pub fn get_element_by_id<T>(id: &str) -> T 
where 
    T: JsCast
{
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.get_element_by_id(id).unwrap().unchecked_into::<T>()
}

pub fn is_err<I, O>(action: Action<I, Response<O>>) -> bool {
    action.value().with(|value|{
        if let Some(response) = value {
            if response.is_err() {
                true
            } else {
                false
            }
        } else {
            false
        }
        
    })
}
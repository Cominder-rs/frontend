#![feature(unboxed_closures)]

use leptos::*;
pub mod apis;
mod components;
mod core;
mod shared;
mod stores;
pub mod utils;
mod views;
use crate::core::router::AppRouter;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
   
    view! { cx,
        <AppRouter />
    }
}

#![feature(unboxed_closures)]

use leptos::*;
mod app;
mod components;
mod core;
mod shared;
mod stores;
mod views;
pub mod apis;
pub mod utils;
use crate::core::{router::AppRouter, router::AppRouterProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    println!("Hui");
    view! { cx,
        <AppRouter />
    }
}

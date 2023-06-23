use leptos::*;
use crate::components::index::Index;

#[component]
pub fn IndexView(cx: Scope) -> impl IntoView {
    view! {cx, <Index />}
}
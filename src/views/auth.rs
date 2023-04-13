use leptos::*;
use crate::components::auth::{AuthComponent, AuthComponentProps};
use crate::stores::auth_store::AuthStore;

#[component]
pub fn AuthView(cx: Scope) -> impl IntoView {
    let auth_store = AuthStore::init(cx);

    provide_context(cx, auth_store);

    view! { cx,
        <AuthComponent />
    }
}

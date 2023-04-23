use crate::views::{AuthView, AuthViewProps};
use leptos::*;
use leptos_router::*;

use tonic_web_wasm_client::Client;

use users_proto::users_client::UsersClient;

const USERS_SERVICE: &'static str = "http://localhost:7777";

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    let users_client = UsersClient::new(Client::new(USERS_SERVICE.to_string()));

    provide_context(cx, users_client);
    
    view! {cx,
        <Router>
            <Routes>
                <Route path="/auth" view=|cx| view! {cx, <AuthView />} />
            </Routes>
        </Router>
    }
}

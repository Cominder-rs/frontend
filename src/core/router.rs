use crate::{views::AuthView, components::notifier::Notifier, stores::notifier_store::NotifierStore};
use leptos::*;
use leptos_router::*;
use crate::apis::USERS_SERVICE;
use tonic_web_wasm_client::Client;
use users_proto::auth_client::AuthClient;

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    let auth_client: StoredValue<AuthClient<Client>> = store_value(cx, AuthClient::new(Client::new(USERS_SERVICE.to_string())));
    provide_context(cx, auth_client);

    let notifier_store = NotifierStore::init(cx);
    provide_context(cx, notifier_store);
    view! {cx,
        <Router>
            <Routes>
                <Route path="/auth" view=|cx| view! {cx, <AuthView />} />
            </Routes>
            <Notifier />
        </Router>
    }
}

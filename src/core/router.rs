use crate::apis::USERS_SERVICE;
use crate::views::IndexView;
use crate::{
    components::notifier::Notifier, stores::notifier_store::NotifierStore, views::AuthView,
};
use leptos::*;
use leptos_router::*;
use tonic::codegen::CompressionEncoding;
use tonic_web_wasm_client::Client;

use users_proto::{users_v1_client::UsersV1Client, auth_client::AuthClient};

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    let auth_client: StoredValue<AuthClient<Client>> = store_value(
        cx,
        AuthClient::new(Client::new(USERS_SERVICE.to_string()))
            .accept_compressed(CompressionEncoding::Gzip),
    );
    provide_context(cx, auth_client);

    let users_v1_client = store_value(
        cx,
        UsersV1Client::new(Client::new(USERS_SERVICE.to_string()))
            .accept_compressed(CompressionEncoding::Gzip),
    );
    provide_context(cx, users_v1_client);
 
    let notifier_store = NotifierStore::init(cx);
    provide_context(cx, notifier_store);
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=|cx| view! {cx, <IndexView />} />
                <Route path="/auth" view=|cx| view! {cx, <AuthView />} />
            </Routes>
            <Notifier />
        </Router>
    }
}

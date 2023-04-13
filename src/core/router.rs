use crate::views::{AuthView, AuthViewProps};
use leptos::*;
use leptos_router::*;

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    let shit = "hui";

    view! {cx,
        <Router>
            <Routes>
                <Route path="/auth" view=|cx| view! {cx, <AuthView />} />
            </Routes>
        </Router>
    }
}

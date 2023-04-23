

use leptos::*;

use frontend::App;
use frontend::AppProps;
fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    })
}

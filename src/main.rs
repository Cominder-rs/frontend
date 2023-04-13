use leptos::*;

use frontend::App;
use frontend::AppProps;
fn main() {
    _ = console_log::init_with_level(log::Level::Trace);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    })
}

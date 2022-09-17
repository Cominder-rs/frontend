use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class={classes!("shit")}>{ "Hello Worldddd!" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}

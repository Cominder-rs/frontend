use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class={classes!("button", "is-primary")}>
                { "Click me!" }
            </button>
        }
    }
}


fn main() {
    yew::start_app::<App>();
}

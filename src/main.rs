use yew::prelude::*;
use ybc::Button;


struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {


        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            // <ybc::Button>
                { "Click me!" }
            // </ybc::Button>
        }
    }
}


fn main() {
    yew::start_app::<App>();
}

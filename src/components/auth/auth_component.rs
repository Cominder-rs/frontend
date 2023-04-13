
use leptos::{*, ev::{Event, MouseEvent, KeyboardEvent}};

use crate::shared::{Input, InputProps};
use crate::stores::auth_store::AuthStore;


#[component]
pub fn AuthComponent(cx: Scope) -> impl IntoView {
    let auth_store = use_context::<AuthStore>(cx).expect("Getting `AuthStore` context");
    let AuthStore {
        phone_number,
        code,
        country,
    } = auth_store;

    let country_field = create_rw_signal(cx, "".to_string());
    
    view! { cx,
        <div class="auth_page">
            <div class="auth_container">
                <div class="auth_header">
                    <div class="auth_header_text">"Авторизация"</div>
                    <div class="auth_header_subtext">"Пожалуйста, выберите  вашу страну и введите номер телефона"</div>
                </div>
        
                <div class="inputs_container">
                    <div class="intermediate_container">
                        <div
                            dir="auto"
                            data-no-linebreaks="1"
                            contenteditable="true"
                            on:keypress= move |e: KeyboardEvent| {
                                if e.key() == "Enter" {
                                    e.prevent_default()
                                }
                            }
                            // value=country_field
                            // on:input=move |e: Event| {auth_store.set_country(e)}
                            class="auth_input country_field"
                            // on:click=move |e: MouseEvent| {
                            //     let target = e.related_target().unwrap().uncheked_ref::<>();
                                
                            // }
                        >
                            "Shitty code"
                        </div>
                        <label
                            class:auth_input_filled={move || country.get().is_some()}
                            class="auth_label"
                        >
                            "Страна"
                        </label>

                    </div>
                    <div class="intermediate_container">
                        <Input
                            on:input=move |e: Event| {auth_store.set_phone_number(e)}
                            value=phone_number
                            class="auth_input"
                        />
                        <label 
                            class="auth_label"
                            class:auth_input_filled={move || !phone_number.get().is_empty()}
                        >
                            "Номер телефона"
                        </label>
                    </div>
                </div>
                <button class="button is-primary auth_input auth_button">"Далее"</button>
               
            </div>
        </div>
    }
}

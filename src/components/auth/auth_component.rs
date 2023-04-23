use leptos::{
    ev::{Event, KeyboardEvent, MouseEvent},
    html::Div,
    *,
};
use tonic_web_wasm_client::Client;
use users_proto::CountryCode;

use crate::stores::auth_store::{AuthStore, CountryFieldEvent};
use crate::{
    core::types::UsersClient,
    shared::{Input, InputProps},
    utils::is_err,
};
use gloo_console::log;



#[component]
pub fn AuthComponent(cx: Scope) -> impl IntoView {
    let auth_store = use_context::<AuthStore>(cx).expect("Getting `AuthStore` context");
    let users_client = use_context::<UsersClient>(cx).expect("Getting `UsersClient` context");

    let AuthStore {
        phone_number,
        code,
        country,
        country_field,
        country_resource,
        send_phone_number,
        ..
    } = auth_store;


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
                            id="country_field"
                            on:paste=move |e| {auth_store.on_input_country(CountryFieldEvent::OnPaste(e))}
                            on:keyup = move |e| {auth_store.on_input_country(CountryFieldEvent::OnKeydown(e))}
                            on:keydown = move |e| {
                                if e.key_code() == 13 {
                                    e.prevent_default()
                                }
                            }
                            on:click = move |_| {auth_store.toggle_countries(true)}
                            on:focusout = move |_| {auth_store.toggle_countries(false)}
                            class:loh = move || {
                                send_phone_number.value().with(|shit| {
                                    is_err(shit)                                    
                                })
                            }
                            class="auth_input country_field"
                            class:country_field_filled = move || {!country_field().is_empty()} 
                        >
                        </div>
                        <label
                            class:auth_input_filled=move || {!country_field().is_empty()}
                            class="auth_label"
                        >
                            "Страна"
                        </label>
                        <i class="fa-solid fa-chevron-down arrow_up"></i>
                        <div class="borders_div" />

                        <ul class="countries" id="countries" on:mousedown=move |e| {auth_store.pick_country(e)}>
                            <li class="country" code={CountryCode::At as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇦🇹"}</div>
                                    <div class="country_name">{"Австрия"}</div>
                                </div>        
                                <div class="phone_code">{"+43"}</div>
                            </li>

                            <li class="country" code={CountryCode::By as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇧🇾"}</div>
                                    <div class="country_name">{"Беларусь"}</div>
                                </div>        
                                <div class="phone_code">{"+375"}</div>
                            </li>
                            <li class="country" code={CountryCode::Be as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇧🇪"}</div>
                                    <div class="country_name">{"Бельгия"}</div>
                                </div>        
                                <div class="phone_code">{"+32"}</div>
                            </li>
                            <li class="country" code={CountryCode::Ca as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇨🇦"}</div>
                                    <div class="country_name">{"Канада"}</div>
                                </div>        
                                <div class="phone_code">{"+1"}</div>
                            </li>
                            <li class="country" code={CountryCode::Cn as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇨🇳"}</div>
                                    <div class="country_name">{"Китай"}</div>
                                </div>        
                                <div class="phone_code">{"+86"}</div>
                            </li>
                            <li class="country" code={CountryCode::Dk as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇩🇰"}</div>
                                    <div class="country_name">{"Дания"}</div>
                                </div>        
                                <div class="phone_code">{"+45"}</div>
                            </li>
                            <li class="country" code={CountryCode::De as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇩🇪"}</div>
                                    <div class="country_name">{"Германия"}</div>
                                </div>        
                                <div class="phone_code">{"+49"}</div>
                            </li>
                            <li class="country" code={CountryCode::Fi as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇫🇮"}</div>
                                    <div class="country_name">{"Финляндия"}</div>
                                </div>        
                                <div class="phone_code">{"+358"}</div>
                            </li>
                            <li class="country" code={CountryCode::Fr as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇫🇷"}</div>
                                    <div class="country_name">{"Франция"}</div>
                                </div>        
                                <div class="phone_code">{"+33"}</div>
                            </li>
          
                            <li class="country" code={CountryCode::Gb as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇬🇧"}</div>
                                    <div class="country_name">{"Великобритания"}</div>
                                </div>        
                                <div class="phone_code">{"+44"}</div>
                            </li>
                            <li class="country" code={CountryCode::Ge as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇬🇪"}</div>
                                    <div class="country_name">{"Грузия"}</div>
                                </div>        
                                <div class="phone_code">{"+995"}</div>
                            </li>
                            <li class="country" code={CountryCode::In as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇮🇳"}</div>
                                    <div class="country_name">{"Индия"}</div>
                                </div>        
                                <div class="phone_code">{"+91"}</div>
                            </li>
                            <li class="country" code={CountryCode::Id as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇮🇩"}</div>
                                    <div class="country_name">{"Индонезия"}</div>
                                </div>        
                                <div class="phone_code">{"+62"}</div>
                            </li>
                            <li class="country" code={CountryCode::It as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇮🇹"}</div>
                                    <div class="country_name">{"Италия"}</div>
                                </div>        
                                <div class="phone_code">{"+39"}</div>
                            </li>
                            <li class="country" code={CountryCode::Jp as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇯🇵"}</div>
                                    <div class="country_name">{"Япония"}</div>
                                </div>        
                                <div class="phone_code">{"+81"}</div>
                            </li>
                            <li class="country" code={CountryCode::Kz as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇰🇿"}</div>
                                    <div class="country_name">{"Казахстан"}</div>
                                </div>        
                                <div class="phone_code">{"+7"}</div>
                            </li>
                            <li class="country" code={CountryCode::Ru as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇷🇺"}</div>
                                    <div class="country_name">{"Россия"}</div>
                                </div>        
                                <div class="phone_code">{"+7"}</div>
                            </li>
                            <li class="country" code={CountryCode::Tr as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇹🇷"}</div>
                                    <div class="country_name">{"Турция"}</div>
                                </div>        
                                <div class="phone_code">{"+90"}</div>
                            </li>
                            <li class="country" code={CountryCode::Ua as i32}>
                                <div class="country_overlap" />
                                <div class="country_left">
                                    <div class="flag">{"🇺🇦"}</div>
                                    <div class="country_name">{"Украина"}</div>
                                </div>        
                                <div class="phone_code">{"+380"}</div>
                            </li>
                        </ul>
                    </div>
                    <div class="intermediate_container">
                        <Input
                            on:input=move |e: Event| {phone_number.set(event_target_value(&e))}
                            value=phone_number
                            class="auth_input"
                            id="phone_number_input"
                        />
                        <label
                            for="phone_number_input"
                            class="auth_label"
                            class:auth_input_filled={move || !phone_number.get().is_empty()}
                        >
                            "Номер телефона"
                        </label>
                        <div class="borders_div" />
                    </div>
                </div>
                <button 
                    class="button is-primary auth_input auth_button"
                    on:click = move |_| {send_phone_number.dispatch(phone_number.get_untracked())} 
                >
                    "Далее"
                </button>
            </div>
        </div>
    }
}

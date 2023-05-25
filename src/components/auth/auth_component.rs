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

pub const COUNTRIES: [(CountryCode, &'static str, &'static str, &'static str); 19] = [
    (CountryCode::At, "🇦🇹", "Австрия",         "+43"),
    (CountryCode::By, "🇧🇾", "Беларусь",        "+375"),
    (CountryCode::Be, "🇧🇪", "Бельгия",         "+32"),
    (CountryCode::Ca, "🇨🇦", "Канада",          "+1"),
    (CountryCode::Cn, "🇨🇳", "Китай",           "+86"),
    (CountryCode::Dk, "🇩🇰", "Дания",           "+45"),
    (CountryCode::De, "🇩🇪", "Германия",        "+49"),
    (CountryCode::Fi, "🇫🇮", "Финляндия",       "+358"),
    (CountryCode::Fr, "🇫🇷", "Франция",         "+33"),
    (CountryCode::Gb, "🇬🇧", "Великобритания",  "+44"),
    (CountryCode::Ge, "🇬🇪", "Грузия",          "+995"),
    (CountryCode::In, "🇮🇳", "Индия",           "+91"),
    (CountryCode::Id, "🇮🇩", "Индонезия",       "+62"),
    (CountryCode::It, "🇮🇹", "Италия",          "+39"),
    (CountryCode::Jp, "🇯🇵", "Япония",          "+81"),
    (CountryCode::Kz, "🇰🇿", "Казахстан",       "+7"),
    (CountryCode::Ru, "🇷🇺", "Россия",          "+7"),
    (CountryCode::Tr, "🇹🇷", "Турция",          "+90"),
    (CountryCode::Ua, "🇺🇦", "Ураина",          "+380"),
];
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
        request_status_indicator,
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
                            on:paste=move |e| auth_store.on_input_country(CountryFieldEvent::OnPaste(e))
                            on:keyup = move |e| auth_store.on_input_country(CountryFieldEvent::OnKeydown(e))
                            on:keydown = move |e| {
                                if e.key_code() == 13 || e.key_code() == 9 {
                                    e.prevent_default()
                                }
                            }
                            on:click = move |_| auth_store.toggle_countries(true)
                            on:focusout = move |_| auth_store.toggle_countries(false)
                            on:focusin = move |_| auth_store.toggle_countries(false)
                            class="auth_input country_field"
                            class:country_field_filled = move || !country_field().is_empty() 
                        />
                        <label
                            class:auth_input_filled=move || !country_field().is_empty()
                            class="auth_label"
                        >
                            "Страна"
                        </label>
                        <i class="fa-solid fa-chevron-down arrow_up"></i>
                        <div class="borders_div" />

                        <ul class="countries" id="countries" on:mousedown=move |e| auth_store.pick_country(e)>
                            {   
                                COUNTRIES.iter().map(|country| {
                                    view! { cx,
                                        <Country
                                            code=country.0
                                            flag=country.1
                                            country_name=country.2
                                            phone_code=country.3
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()
                            }
        
                        </ul>
                    </div>
                    <div class="intermediate_container">
                        <input
                            on:paste = move |e| auth_store.check_paste_phone_number(e)
                            on:input=move |e: Event| auth_store.set_phone_number(e)
                            on:keydown = move |e: KeyboardEvent| auth_store.check_input_phone_number(e)
                            value=phone_number
                            prop:value=phone_number
                            class="auth_input input"
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


#[component]
fn Country(
    cx: Scope,
    code: CountryCode,
    flag: &'static str,
    country_name: &'static str,
    phone_code: &'static str,
) -> impl IntoView {
    view! { cx, 
        <li class="country" code={code as i32}>
            <div class="country_overlap" />
            <div class="country_left">
                <div class="flag">{flag}</div>
                <div class="country_name">{country_name}</div>
            </div>        
            <div class="phone_code">{phone_code}</div>
        </li>
    }
}
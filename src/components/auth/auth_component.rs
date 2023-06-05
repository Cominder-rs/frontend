use crate::utils::{input_mask, map_status_and_err, map_status_and_err2};
use leptos::*;
use users_errors::AuthError;
use users_proto::CountryCode;

use crate::stores::auth_store::{AuthStage, AuthStore, CountryFieldEvent};

pub const COUNTRIES: [(CountryCode, &str, &str, &str); 19] = [
    (CountryCode::At, "🇦🇹", "Австрия", "+43"),
    (CountryCode::By, "🇧🇾", "Беларусь", "+375"),
    (CountryCode::Be, "🇧🇪", "Бельгия", "+32"),
    (CountryCode::Ca, "🇨🇦", "Канада", "+1"),
    (CountryCode::Cn, "🇨🇳", "Китай", "+86"),
    (CountryCode::Dk, "🇩🇰", "Дания", "+45"),
    (CountryCode::De, "🇩🇪", "Германия", "+49"),
    (CountryCode::Fi, "🇫🇮", "Финляндия", "+358"),
    (CountryCode::Fr, "🇫🇷", "Франция", "+33"),
    (CountryCode::Gb, "🇬🇧", "Великобритания", "+44"),
    (CountryCode::Ge, "🇬🇪", "Грузия", "+995"),
    (CountryCode::In, "🇮🇳", "Индия", "+91"),
    (CountryCode::Id, "🇮🇩", "Индонезия", "+62"),
    (CountryCode::It, "🇮🇹", "Италия", "+39"),
    (CountryCode::Jp, "🇯🇵", "Япония", "+81"),
    (CountryCode::Kz, "🇰🇿", "Казахстан", "+7"),
    (CountryCode::Ru, "🇷🇺", "Россия", "+7"),
    (CountryCode::Tr, "🇹🇷", "Турция", "+90"),
    (CountryCode::Ua, "🇺🇦", "Украина", "+380"),
];
#[component]
pub fn AuthComponent(cx: Scope) -> impl IntoView {
    let auth_store = use_context::<AuthStore>(cx).expect("Getting `AuthStore` context");

    let AuthStore {
        phone_number,
        country_field,
        get_country,
        send_phone_number,
        auth_stage,
        invalid_phone_number,
        confirmation_code,
        send_code_again_timer,
        is_confirmation_code_invalid,
        send_confirmation_code,
        login,
        firstname,
        lastname,
        city,
        create_user,
        invalid_login,
        busy_login,
        invalid_firstname,
        invalid_lastname,
        invalid_city,
        ..
    } = auth_store;
    get_country.dispatch(());
    view! { cx,
        <div class="auth_page">
            <div class="auth_container" class:slide-left=move || {auth_stage() == AuthStage::ConfirmationCode || auth_stage() == AuthStage::BasicInfo}>
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
                            on:input=move |e| auth_store.set_phone_number(e)
                            on:paste=move |e| {
                                let value = event_target_value(&e);
                                if let Some(char) = value.chars().collect::<Vec<_>>().first() {
                                    if *char != '+' {
                                        phone_number.update(|prev| {prev.insert(0, '+')})
                                    }
                                } else {
                                    phone_number.update(|prev| {prev.insert(0, '+')})
                                }
                            }
                            on:keypress=move |e| {
                                if e.key_code() == 13 {
                                    send_phone_number.dispatch(phone_number.get_untracked())
                                }
                            }
                            value=phone_number
                            prop:value=phone_number
                            class="auth_input input"
                            class:invalid-input=invalid_phone_number
                            id="phone_number_input"
                        />
                        <label
                            for="phone_number_input"
                            class="auth_label"
                            class:auth_input_filled={move || !phone_number.get().is_empty()}
                         >
                            {move || if invalid_phone_number() {
                                "Неверный номер телефона"
                            } else {
                                "Номер телефона"
                            }}
                        </label>
                        <div class="borders_div" />
                    </div>
                </div>
                <button
                    attr:disabled=move || {send_phone_number.pending()() || get_country.pending()()}
                    class="button is-primary auth_input auth_button"
                    on:click = move |_| {send_phone_number.dispatch(phone_number.get_untracked())}
                >
                    "Далее"
                </button>
            </div>
            <div
                class="auth_container confirmation-container"
                class:slide-center=move|| {auth_stage() == AuthStage::ConfirmationCode}
                class:slide-left=move || {auth_stage() == AuthStage::BasicInfo}
            >
                <div class="auth_header">
                    <div class="auth_header_text">"Введите код"</div>
                    <div class="auth_header_subtext">{"На номер"}<p class="phone-number-text">" "{phone_number}" "</p>{"был выслан проверочный код"}</div>
                </div>
                <div class="intermediate_container confirmation-middle-container">
                    <input
                        on:keypress=move|e| {
                            if e.key_code() == 13 {
                                send_confirmation_code.dispatch(())
                            }
                        }
                        on:input=move|e| {auth_store.set_confirmation_code(e); is_confirmation_code_invalid.set(false)}
                        class:invalid-input=is_confirmation_code_invalid
                        value=confirmation_code
                        prop:value=confirmation_code
                        class="auth_input input"
                        id="confirmation-code"
                    />
                    <label
                        for="confirmation-code"
                        class="auth_label"
                        class:auth_input_filled={move || !confirmation_code.get().is_empty()}
                     >
                        {move || {
                            if is_confirmation_code_invalid() {
                                "Неверный код"
                            } else {
                                "Код"
                            }
                        }}
                    </label>
                    <div class="borders_div" />
                    <p
                        class="send-code-again"
                        class:disabled={move || send_code_again_timer().is_some()}
                        on:click=move |_| auth_store.send_code_again()
                    >
                        {move || {
                            if let Some(timer) = send_code_again_timer() {
                                let text = "Отправить код повторно: 00:".to_owned();
                                if timer <= 9 {
                                    text + "0" + &timer.to_string()
                                } else {
                                    text + &timer.to_string()
                                }
                            } else {
                                "Отправить код повторно".to_owned()
                            }
                        }}
                    </p>
                </div>
                <div class="buttons-container">
                    <button
                        prop:disabled=move || send_confirmation_code.pending()()
                        class="button is-primary auth_input auth_button"
                        on:click = move |_| auth_store.auth_stage.set(AuthStage::Main)
                    >
                        <i class="fa-solid fa-arrow-left"></i>
                    </button>
                    <button
                        prop:disabled=move || send_confirmation_code.pending()()
                        class="button is-primary auth_input auth_button"
                        on:click = move |_| send_confirmation_code.dispatch(())
                    >
                        "Подтвердить"
                    </button>
                </div>
            </div>
            <div
                class="auth_container basic-info"
                class:slide-center=move|| {auth_stage() == AuthStage::BasicInfo}
                class:slide-right=move || {auth_stage() == AuthStage::Main || auth_stage() == AuthStage::ConfirmationCode}
            >
                <div class="auth_header">
                    <div class="auth_header_text">"Немного о себе"</div>
                    <div class="auth_header_subtext">"Заполните следующие поля для завершения регистрации"</div>
                </div>
                    <form class="inputs_container">
                        <div class="intermediate_container">
                            <input
                                on:input=move|e| {invalid_login.set(false); input_mask(login, e, r"^(\p{L}(?<!\s)\s?){0,30}$", Some("u"))}
                                prop:value=login
                                class:invalid-input=invalid_login
                                class="auth_input input"
                                id="login"

                            />
                            <label
                                for="login"
                                class="auth_label"
                                class:auth_input_filled={move || !login.get().is_empty()}
                            >
                                {move || {
                                    if invalid_login() {
                                        return "Не менее двух символов"
                                    }

                                    if busy_login() {
                                        return "Логин занят"
                                    }
                                    "Логин"    
                                }}
                            </label>
                            <div class="borders_div" />
                        </div>
                        <div class="intermediate_container">
                            <input
                                class:invalid-input=invalid_firstname
                                class="auth_input input"
                                on:input=move|e| {invalid_firstname.set(false); input_mask(firstname, e, r"^(\p{L}(?<!\s)\s?){0,30}$", Some("u"))}
                                prop:value=firstname
                                id="firstname"
                            />
                            <label
                                for="firstname"
                                class="auth_label"
                                class:auth_input_filled={move || !firstname.get().is_empty()}
                            >
                                {move || {
                                    if invalid_firstname() {
                                        return "Не двух символов"
                                    }
                                    "Имя"
                                }}
                            </label>
                            <div class="borders_div" />
                        </div>
                        <div class="intermediate_container">
                            <input
                                class:invalid-input=invalid_lastname
                                on:input=move|e| {invalid_lastname.set(false); input_mask(lastname, e, r"^(\p{L}(?<!\s)\s?){0,30}$", Some("u"))}
                                prop:value=lastname
                                class="auth_input input"
                                id="lastname"
                            />
                            <label
                                for="lastname"
                                class="auth_label"
                                class:auth_input_filled={move || !lastname.get().is_empty()}
                            >
                                {move || {
                                    if invalid_lastname() {
                                        return "Не двух символов"
                                    }
                                    "Фамилия"
                                }}
                            </label>
                            <div class="borders_div" />
                        </div>
                        <div class="intermediate_container">
                            <input
                                on:input=move|e| {invalid_city.set(false); input_mask(city, e, r"^(\p{L}(?<!\s)\s?){0,30}$", Some("u"))}
                                class:invalid-input=invalid_city
                                class="auth_input input"
                                prop:value=city
                                id="city"
                            />
                            <label
                                for="city"
                                class="auth_label"
                                class:auth_input_filled={move || !city.get().is_empty()}
                            >
                                {move || {
                                    if invalid_city() {
                                        return "Не двух символов"
                                    }
                                    "Город"
                                }}
                            </label>
                            <div class="borders_div" />
                        </div>
                        <button
                            // on:submit=move|e| e.prevent_default()
                            on:click=move|e| {e.prevent_default(); create_user.dispatch(())}
                            class="button is-primary auth_input auth_button send-basic-info"
                        >
                            "Подтвердить"
                        </button>
                </form>
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

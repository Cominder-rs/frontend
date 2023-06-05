use crate::core::types::Response;
use crate::stores::notifier_store::NotificationVariant;
use crate::utils::{get_element_by_id, match_media};
use crate::{apis::USERS_SERVICE, utils::handle_response};

use leptos::html::Input;
use leptos::{
    ev::{Event, MouseEvent},
    html::Div,
    *,
};
use leptos_router::{use_navigate, NavigateOptions};
use users_proto::{RegistryStatus, NewUser};
use wasm_bindgen::prelude::Closure;

use crate::components::auth::COUNTRIES;
use js_sys::RegExp;
use tonic::{Code, Status};
use tonic_web_wasm_client::Client;
use users_proto::{
    auth_client::AuthClient, ConfirmationCode, CountryCode, PhoneNumber,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlInputElement, HtmlUListElement, InputEvent, KeyboardEvent, SubmitEvent};

use super::notifier_store::NotifierStore;
use users_errors::AuthError;

pub enum CountryFieldEvent {
    OnPaste(Event),
    OnKeydown(KeyboardEvent),
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum AuthStage {
    Main,
    ConfirmationCode,
    BasicInfo,
}

#[derive(Clone, Copy)]
pub struct AuthStore {
    pub auth_client: StoredValue<AuthClient<Client>>,
    pub code: RwSignal<String>,
    pub phone_number: RwSignal<String>,
    pub country: RwSignal<CountryCode>,
    pub country_field: RwSignal<String>,
    pub get_country: Action<(), ()>,
    pub send_phone_number: Action<String, ()>,
    pub request_status_indicator: RwSignal<bool>,
    pub auth_stage: RwSignal<AuthStage>,
    pub main_stage_ref: NodeRef<Div>,
    pub confirmation_code_ref: NodeRef<Div>,
    pub basic_info_ref: NodeRef<Div>,
    pub invalid_phone_number: RwSignal<bool>,
    pub slide_main_stage: RwSignal<bool>,
    pub slide_confirmation_stage: RwSignal<bool>,
    pub slide_info_stage: RwSignal<bool>,
    pub confirmation_code: RwSignal<String>,
    pub send_code_again_timer: RwSignal<Option<i32>>,
    pub random_key1: StoredValue<String>,
    pub random_key2: StoredValue<String>,
    pub is_confirmation_code_invalid: RwSignal<bool>,
    pub send_confirmation_code: Action<(), ()>,
    pub login: RwSignal<String>,
    pub firstname: RwSignal<String>,
    pub lastname: RwSignal<String>,
    pub city: RwSignal<String>,
    pub create_user: Action<(), ()>,
    pub invalid_login: RwSignal<bool>,
    pub invalid_firstname: RwSignal<bool>,
    pub invalid_lastname: RwSignal<bool>,
    pub invalid_city: RwSignal<bool>,
    pub busy_login: RwSignal<bool>,
}

impl AuthStore {
    pub fn init(cx: Scope) -> AuthStore {
        let random_key1 = store_value(cx, String::new());
        let random_key2 = store_value(cx, String::new());
        let send_code_again_timer = create_rw_signal(cx, None);
        let confirmation_code = create_rw_signal(cx, String::new());
        let slide_main_stage = create_rw_signal(cx, false);
        let slide_confirmation_stage = create_rw_signal(cx, false);
        let slide_info_stage = create_rw_signal(cx, false);
        let invalid_phone_number = create_rw_signal(cx, false);
        let main_stage_ref = create_node_ref(cx);
        let confirmation_code_ref = create_node_ref(cx);
        let basic_info_ref = create_node_ref(cx);
        let phone_number = create_rw_signal(cx, String::default());
        let code = create_rw_signal(cx, String::default());
        let country = create_rw_signal(cx, CountryCode::Unknown);
        let country_field = create_rw_signal(cx, String::default());
        let auth_client = use_context::<StoredValue<AuthClient<Client>>>(cx).unwrap();
        let mut notifier_store = use_context::<NotifierStore>(cx).unwrap();
        let request_status_indicator = create_rw_signal(cx, false);
        let auth_stage = create_rw_signal(cx, AuthStage::Main);
        let is_confirmation_code_invalid = create_rw_signal(cx, false);
        let firstname = create_rw_signal(cx, String::new());
        let lastname= create_rw_signal(cx, String::new());
        let login = create_rw_signal(cx, String::new());
        let city= create_rw_signal(cx, String::new());

        let invalid_firstname = create_rw_signal(cx, false);
        let invalid_lastname = create_rw_signal(cx, false);
        let invalid_login = create_rw_signal(cx, false);
        let invalid_city = create_rw_signal(cx, false);
        let busy_login = create_rw_signal(cx, false); 

        let get_country = create_action(cx, move |_: &()| async move {
            let mut auth_client = auth_client.get_value();
            let res = auth_client.get_country(()).await;
            if let Ok(res) = res {
                let code = res.into_inner().code;

                for country in COUNTRIES {
                    if country.0 as i32 == code {
                        country_field.set(country.2.to_string());
                        phone_number.set(country.3.to_string() + " (");
                        let country_field = get_element_by_id::<HtmlDivElement>("country_field");
                        country_field.set_text_content(Some(country.2));
                        if !match_media("(max-width: 768px)") {
                            _ = get_element_by_id::<HtmlDivElement>("phone_number_input").focus();
                        }
                    }
                }
            }
        });

        let send_phone_number = create_action(cx, move |phone_number: &String| {
            let mut phone_number_chars: Vec<char> = phone_number.chars().collect();

            phone_number_chars.retain(|i| *i != '-' || *i != '(' || *i != ')' || *i != ' ');

            let phone_number = PhoneNumber {
                phone_number: phone_number_chars.iter().cloned().collect::<String>(),
            };
            async move {
                let mut auth_client = auth_client.get_value();
                cfg_if::cfg_if! {
                    if #[cfg(not(feature = "prod"))] {
                        let response = handle_response(cx, auth_client.send_phone_number_dev(phone_number).await);
                        match response {
                            Ok(response) => {
                                notifier_store.notify(cx, NotificationVariant::Info, "Ваш код: ".to_string() + response.get_ref().code.as_ref(), Some(6000));
                                random_key1.set_value(response.into_inner().random_key);
                                auth_stage.set(AuthStage::ConfirmationCode);
                                if !match_media("(max-width: 768px)") {
                                    _ = get_element_by_id::<HtmlDivElement>("confirmation-code").focus();
                                }
                            },
                            Err(error) => {
                                let message = error.message().into();
                                if let AuthError::InvalidPhoneNumber = message {
                                    invalid_phone_number.set(true);
                                }
                            }

                        }
                    } else {
                        auth_client.send_phone_number(phone_number).await;
                    }
                }
            }
        });

        let send_confirmation_code = create_action(cx, move |_: &()| {
            let confimation_code = ConfirmationCode {
                code: confirmation_code.get_untracked(),
                random_key: random_key1.get_value(),
            };

            async move {
                let mut auth_client = auth_client.get_value();
                let response: Response<RegistryStatus> = handle_response(
                    cx,
                    auth_client.send_confirmation_code(confimation_code).await,
                );
                match response {
                    Ok(res) => {
                        let registry_status = res.into_inner();
                        if registry_status.is_done {
                            let token = registry_status.token.unwrap();
                            let storage = window().local_storage().unwrap().unwrap();
                            _ = storage.set_item("token", format!("Bearer {token}").as_str());
                            _ = use_navigate(cx)("/", NavigateOptions::default());
                        } else {
                            auth_stage.set(AuthStage::BasicInfo);
                            random_key2.set_value(registry_status.random_key.unwrap());
                        }
                    }
                    Err(status) => {
                        if let AuthError::InvalidConfirmationCode = status.message().into() {
                            is_confirmation_code_invalid.set(true)
                            
                        }
                    }
                }
            }
        });
        let create_user = create_action(cx, move |_: &()| {

            let login = login.get_untracked();
            let firstname = firstname.get_untracked();
            let lastname = lastname.get_untracked();
            let city = city.get_untracked();

            let new_uesr = NewUser {
                username: login,
                firstname,
                lastname,
                city,
                random_key: random_key2.get_value(),
            };

            async move {
                let res = handle_response(cx, auth_client.get_value().create_user(new_uesr).await);
                match res {
                    Ok(res) => {
                        let token = res.into_inner().token;
                        let storage = window().local_storage().unwrap().unwrap();
                        _ = storage.set_item("token", format!("Bearer {token}").as_str());
                        _ = use_navigate(cx)("/", NavigateOptions::default());
                    },
                    Err(status) => {
                        match status.message().into() {
                            AuthError::UsernameBusy => busy_login.set(true),
                            AuthError::InvalidFirstname => invalid_firstname.set(true),
                            AuthError::InvalidLastname => invalid_lastname.set(true),
                            AuthError::InvalidCity => invalid_city.set(true),
                            AuthError::InvalidUsername => invalid_login.set(true),
                            _ => ()
                        }
                    },
                }
            }
        });

        AuthStore {
            send_phone_number,
            auth_client,
            phone_number,
            code,
            country,
            country_field,
            request_status_indicator,
            get_country,
            auth_stage,
            main_stage_ref,
            confirmation_code_ref,
            basic_info_ref,
            invalid_phone_number,
            slide_main_stage,
            slide_confirmation_stage,
            slide_info_stage,
            confirmation_code,
            send_code_again_timer,
            random_key1,
            random_key2,
            is_confirmation_code_invalid,
            send_confirmation_code,
            login,
            firstname,
            lastname,
            city,
            create_user,
            invalid_login,
            invalid_firstname,
            invalid_lastname,
            invalid_city,
            busy_login,
        }
    }

    pub fn set_phone_number(&self, event: Event) {
        let last_input_char = event.clone().unchecked_into::<InputEvent>().data();
        let value = event_target_value(&event);

        let allowed_keys = RegExp::new(
            r"^\+?(\d+(?<=\d)\s?\(?\d*((?<=\d+)\))?)?((?<=\d+|\(\d+\))\s?)?(\d*)?((?<=\d+)-?)?(\d+)?((?<=\d+)-?)?(\d*)?$",
            "",
        );
        if allowed_keys.test(&value) {
            self.invalid_phone_number.set(false);
            if self.phone_number.get_untracked().is_empty() && value != "+" && value.len() <= 1 {
                self.phone_number.set("+".to_owned() + &value);
            } else if self.country_field.get_untracked() == "Россия" {
                if let Some(last_input_char) = last_input_char {
                    let mask = "+9 (999) 999-99-99";

                    if let Some(last_input_char) = last_input_char.chars().last() {
                        let mask_chars: Vec<_> = mask.chars().collect();
                        let mut new_value = value;
                        loop {
                            let next_mask_char = mask_chars.get(new_value.len());

                            if let Some(next_mask_char) = next_mask_char {
                                if *next_mask_char != '9'
                                    && *next_mask_char != '+'
                                    && last_input_char != '('
                                    && last_input_char != ')'
                                    && last_input_char != ' '
                                    && last_input_char != '-'
                                {
                                    if new_value.len() == 3 {
                                        let mut to_insert = " ".to_string();
                                        to_insert.push(*next_mask_char);
                                        new_value.insert_str(new_value.len() - 1, &to_insert);
                                    } else {
                                        new_value.push(*next_mask_char);
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        self.phone_number.set(new_value);
                    }
                } else {
                    self.phone_number.set(value);
                }
            } else {
                self.phone_number.set(value);
            }

            let mut counter = 0;
            COUNTRIES.iter().for_each(|country| {
                let mut pattern = country.3.to_owned();
                pattern.push_str(".*");
                pattern.insert(0, '\\');
                let regexp = RegExp::new(&pattern, "");
                let country_field = get_element_by_id::<HtmlDivElement>("country_field");
                if regexp.test(&self.phone_number.get_untracked()) {
                    self.country_field.set(country.2.to_string());
                    country_field.set_text_content(Some(country.2));
                    return;
                }
                counter += 1;
                if counter == COUNTRIES.len() {
                    self.country_field.set("".to_string());
                    country_field.set_text_content(None);
                }
            });
        } else {
            let target = event_target::<HtmlInputElement>(&event);
            target.set_value(&self.phone_number.get_untracked());
        }
    }

    pub fn on_input_country(&self, event: CountryFieldEvent) {
        let target = match event {
            CountryFieldEvent::OnPaste(event) => event.target().unwrap(),
            CountryFieldEvent::OnKeydown(event) => {
                event.prevent_default();
                event.target().unwrap()
            }
        };

        let value = target
            .unchecked_into::<web_sys::HtmlDivElement>()
            .text_content();

        if let Some(value) = value {
            self.country_field.set(value);
            self.set_filter(false);
        } else {
            self.country_field.set(String::default())
        }
    }

    pub fn toggle_countries(&self, on_click: bool) {
        let countries = get_element_by_id::<HtmlDivElement>("countries");
        let classes = countries.class_list();

        if on_click && classes.contains("show_countries") {
            return;
        }

        _ = classes.toggle("show_countries");
    }

    pub fn set_filter(&self, clean: bool) {
        let countries = get_element_by_id::<HtmlDivElement>("countries");
        let children = countries.children();
        let children = js_sys::Array::from(&children);

        let mut counter_hidden = 0;
        for child in children.iter() {
            let child = child.unchecked_into::<HtmlDivElement>();
            let child_classes = child.class_list();
            if clean {
                _ = child_classes.remove_1("is-hidden");
                continue;
            }

            let left_part = child.children().item(1).unwrap();
            let country_name = left_part
                .children()
                .item(1)
                .unwrap()
                .text_content()
                .unwrap();

            let filter = self.country_field.get_untracked().to_lowercase();

            if !country_name.to_lowercase().starts_with(&filter) {
                _ = child_classes.add_1("is-hidden");
                counter_hidden += 1;
            } else {
                _ = child_classes.remove_1("is-hidden");
                counter_hidden -= 1;
            }
            if counter_hidden == 19 {
                self.set_filter(true);
            }
        }
    }

    pub fn pick_country(&self, event: MouseEvent) {
        self.invalid_phone_number.set(false);
        let target: HtmlUListElement = event_target(&event);

        if target.id() == *"countries" {
            return;
        }

        let children = target.parent_element().unwrap().children();

        let country_name = children
            .item(1)
            .unwrap()
            .children()
            .item(1)
            .unwrap()
            .text_content()
            .unwrap();

        let phone_number = children.item(2).unwrap().text_content().unwrap();

        self.country_field.set(country_name.clone());
        let country_field = get_element_by_id::<HtmlDivElement>("country_field");
        country_field.set_text_content(Some(&country_name));
        self.phone_number.set(phone_number);
    }

    pub fn set_confirmation_code(self, event: Event) {
        let value = event_target_value(&event);

        let allowed_keys = RegExp::new(r"^[0-9]{0,6}$", "");
        if allowed_keys.test(&value) {
            self.confirmation_code.set(value);
        } else {
            event_target::<HtmlInputElement>(&event)
                .set_value(&self.confirmation_code.get_untracked());
        }
    }

    pub fn send_code_again(self) {
        if self.send_code_again_timer.get_untracked().is_some() {
            return;
        }

        self.send_phone_number
            .dispatch(self.phone_number.get_untracked());

        self.send_code_again_timer.set(Some(59));
        let callback = Closure::wrap(Box::new(move || {
            self.send_code_again_timer
                .update(|prev| *prev.as_mut().unwrap() -= 1);
        }) as Box<dyn Fn()>);

        let interval_id = window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                &callback.into_js_value().into(),
                1000,
            )
            .unwrap();

        let callback = Closure::wrap(Box::new(move || {
            window().clear_interval_with_handle(interval_id);
            self.send_code_again_timer.set(None);
        }) as Box<dyn Fn()>);

        _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            &callback.into_js_value().into(),
            60000,
        );
    }

    pub fn create_user(&self, event: SubmitEvent) {
    }
}

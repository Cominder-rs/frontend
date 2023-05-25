use crate::apis::USERS_API;
use crate::core::types::UsersClient;
use crate::utils::get_element_by_id;
use gloo_console::log;
use leptos::{
    ev::{Event, MouseEvent},
    *,
};
use tonic::metadata::MetadataValue;
use tonic_web_wasm_client::Client;
use users_proto::{users_client::UsersClient as _UsersClient, Country, CountryCode, PhoneNumber};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlUListElement, KeyboardEvent};
use js_sys::RegExp;
use crate::core::types::Response;
use crate::components::auth::COUNTRIES;

const ALLOWED_KEYS: [&'static str; 18] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "Backspace", "+", "ArrowRight", "ArrowUp", "ArrowDown", "ArrowLeft", "Control", "Shift"];

pub enum CountryFieldEvent {
    OnPaste(Event),
    OnKeydown(KeyboardEvent),
}

#[derive(Clone, Copy)]
pub struct AuthStore {
    pub users_client: StoredValue<UsersClient>,
    pub code: RwSignal<String>,
    pub phone_number: RwSignal<String>,
    pub country: RwSignal<CountryCode>,
    pub country_field: RwSignal<String>,
    pub country_resource: Resource<CountryCode, Response<Country>>,
    pub send_phone_number: Action<String, Response<()>>,
    pub request_status_indicator: RwSignal<bool>
}

impl AuthStore {
    pub fn init(cx: Scope) -> AuthStore {
        let phone_number = create_rw_signal(cx, String::default());
        let code = create_rw_signal(cx, String::default());
        let country = create_rw_signal(cx, CountryCode::Unknown);
        let country_field = create_rw_signal(cx, String::default());
        let users_client = store_value(
            cx,
            _UsersClient::new(Client::new(USERS_API.to_owned()).with_headers(vec![(
                "Authorization".to_owned(),
                "Bearer some-token".to_owned(),
            )])),
        );
        let request_status_indicator = create_rw_signal(cx, false);
        let country_resource =
            create_local_resource(cx, country, move |_| {
                async move {
                    let mut users_client = users_client.get_value();
                    users_client.get_country(()).await
                }
            });

        let send_phone_number = create_action(cx, move |phone_number: &String| {
            let phone_number = PhoneNumber {
                phone_number: phone_number.to_owned()
            };
            async move {
                let mut users_client = users_client.get_value();
                users_client.send_phone_number(phone_number).await
            }
        });
        
        AuthStore {
            send_phone_number,
            users_client,
            phone_number,
            code,
            country,
            country_field,
            country_resource,
            request_status_indicator,
        }
    }

    pub fn check_paste_phone_number(&self, event: Event) {
        let value = event_target_value(&event);
        // event.prevent_default();
        // log!(value);

        let navigator = web_sys::window().unwrap().navigator();
        // let text = navigator.clipboard();
        // log!(text);        
    }

    pub fn check_input_phone_number(&self, event: KeyboardEvent) {
        if event.ctrl_key() {
            return;
        }
               
        let key_code = event.key_code();
        let key = event.key();

                
        if !ALLOWED_KEYS.contains(&key.as_str()) {
            event.prevent_default();
            return;
        }
        if event.key().as_str() == "=" {
            event.prevent_default();
            return;
        }
        
        if key_code == 61 && self.phone_number.get_untracked().len() != 0 {
            event.prevent_default();
            return;
        }

        if self.phone_number.get_untracked().len() == 0 && key_code != 61 {
            self.phone_number.set("+".to_string());
        }
    }

    pub fn set_phone_number(&self, event: Event) {
        // log!(&event);
        let value = event_target_value(&event);
        
        
        self.phone_number.set(value.clone());
        let mut counter = 0;
        COUNTRIES.iter().for_each(|country| {
            
            let mut pattern = country.3.to_owned();
            pattern.push_str(".*");
            pattern.insert(0, '\\');
            let regexp = RegExp::new(&pattern, "");
            let country_field = get_element_by_id::<HtmlDivElement>("country_field");
            if regexp.test(&value) {
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

        if on_click {
            if classes.contains("show_countries") {
                return;
            }
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
        let target: HtmlUListElement = event_target(&event);

        if target.id() == "countries".to_string() {
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
        self.phone_number.update(|pn| {
            *pn = phone_number
        });
    }
}

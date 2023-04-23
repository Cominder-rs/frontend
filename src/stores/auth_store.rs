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
use crate::core::types::Response;

async fn get_country(mut users_client: UsersClient) -> CountryCode {
    let country_code = users_client
        .get_country(())
        .await
        .unwrap_or(tonic::Response::new(Country {
            code: CountryCode::Unknown as i32,
        }))
        .into_inner()
        .code;

    country_code.into()
}

async fn send_phone_number(mut users_client: UsersClient) {
    let phone_number = PhoneNumber {
        phone_number: "+79125224649".to_string(),
    };
    users_client.send_phone_number(phone_number).await.unwrap();
}

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
    pub country_resource: Resource<CountryCode, CountryCode>,
    pub send_phone_number: Action<String, Response<()>>,
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

        let country_resource =
            create_local_resource(cx, country, move |_| get_country(users_client.get_value()));

        let send_phone_number = create_action(cx, move |phone_number: &String| {
            log!("shet");
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
        }
    }

    pub fn set_phone_number(&self, event: Event) {
        self.phone_number
            .update(|phone_number| *phone_number = event_target_value(&event))
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

            let filter = self.country_field.get().to_lowercase();

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
}

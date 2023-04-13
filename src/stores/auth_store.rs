use leptos::{ev::Event, *};

#[derive(Clone, Copy)]
pub enum Country {
    Russia,
}

#[derive(Clone, Copy)]
pub struct AuthStore {
    pub code: RwSignal<String>,
    pub phone_number: RwSignal<String>,
    pub country: RwSignal<Option<Country>>,
}

impl AuthStore {
    pub fn init(
        cx: Scope,
    ) -> AuthStore {
        let phone_number = create_rw_signal(cx, String::default());
        let code = create_rw_signal(cx, String::default());
        let country = create_rw_signal(cx, None);

        AuthStore {
            phone_number,
            code,
            country,
        }
    }

    pub fn set_phone_number(&self, event: Event) {
        self.phone_number
            .update(|phone_number| *phone_number = event_target_value(&event))
    }

    pub fn set_country(&self, country: Country) {
        self.country
            .update(|c| *c = Some(country))

    }
}

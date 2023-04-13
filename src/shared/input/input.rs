use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    value: RwSignal<String>,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] input_type: String,
    #[prop(optional)] placeholder: String,
    #[prop(optional)] id: String,
) -> impl IntoView {
    view! { cx, class="input is-primary shared_input",
        <input
            id=id
            class=class
            type=input_type
            placeholder=placeholder
            prop:value=value
            value=value       
        />
    }
} 
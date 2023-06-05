use leptos::*;

#[component]
pub fn Input(
    cx: Scope,
    value: RwSignal<String>,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] input_type: String,
    #[prop(optional)] placeholder: String,
    #[prop(optional, into)] id: Option<AttributeValue>,
) -> impl IntoView {
    view! { cx,
        <input
            id=id
            class=class
            class:input=true
            class:is-primary=true
            class:shared_input=true
            type=input_type
            placeholder=placeholder
            prop:value=value
            value=value       
        />
    }
} 
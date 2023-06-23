use crate::utils::input_mask;
use leptos::*;
use web_sys::HtmlInputElement;

pub enum InputVariant {
    Classic,
    Textarea,
    Picker(Vec<String>),
}

#[component]
pub fn Input(
    cx: Scope,
    value: RwSignal<String>,
    variant: InputVariant,
    #[prop(optional, into)] class: Option<AttributeValue>,
    #[prop(optional)] input_type: String,
    #[prop(optional)] label: String,
    #[prop(optional)] is_invalid: Option<RwSignal<bool>>,
    #[prop(optional)] invalid_text: Option<RwSignal<String>>,
    /// 1 = regex, 2 = flags
    #[prop(optional)]
    mask: Option<(String, Option<String>)>,

    id: String,
) -> impl IntoView {
    match variant {
        InputVariant::Classic => view! { cx,
            <div class="shared-input-container">
                <input
                    id=id.clone()
                    class={class}
                    class:shared-input=true
                    class:input=true
                    class:shared-input-invalid=
                        {move || {
                            if let Some(is_invalid) = is_invalid { is_invalid() } else { false };
                            if let Some(invalid_text) = invalid_text { !invalid_text().is_empty() } else { false }
                        }
                    }
                    type=input_type
                    prop:value=value
                    value=value
                    on:input=move |e| {
                        if let Some(mask) = &mask {
                            input_mask(value, e, &mask.0, mask.1.as_deref())
                        } else {
                            value.set(event_target_value(&e))
                        };
                        if let Some(is_invalid) = is_invalid {
                            is_invalid.set(false)
                        }
                        if let Some(invalid_text) = invalid_text {
                            invalid_text.update(|prev| prev.clear())

                        }
                    }
                />
                <div />
                <label for=id class:filled-input=move || {!value().is_empty()}>
                    {move || {
                        if let Some(invalid_text) = invalid_text {
                            if !invalid_text().is_empty() {
                                return invalid_text()
                            }
                        }
                        label.clone()
                    }}
                </label>
            </div>
        },

        InputVariant::Textarea => view! { cx,
            <div
                class="shared-input-container shared-textarea-container"
            >
                <textarea
                    id=id.clone()
                    class={class}
                    class:shared-input=true
                    class:textarea=true
                    class:shared-textarea=true
                    class:shared-input-invalid=
                        {move || {
                            if let Some(is_invalid) = is_invalid { is_invalid() } else { false };
                            if let Some(invalid_text) = invalid_text { !invalid_text().is_empty() } else { false }
                        }
                    }
                    type=input_type
                    prop:value=value
                    value=value
                    on:input=move |e| {
                        if let Some(mask) = &mask {
                            input_mask(value, e, &mask.0, mask.1.as_deref())
                        } else {
                            value.set(event_target_value(&e))
                        };
                        if let Some(is_invalid) = is_invalid {
                            is_invalid.set(false)
                        }
                        if let Some(invalid_text) = invalid_text {
                            invalid_text.update(|prev| prev.clear())

                        }
                    }
                />
                <div />
                <label for=id class:filled-input=move || {!value().is_empty()}>
                    {move || {
                        if let Some(invalid_text) = invalid_text {
                            if !invalid_text().is_empty() {
                                return invalid_text()
                            }
                        }
                        label.clone()
                    }}
                </label>
            </div>
        },

        InputVariant::Picker(variants) => view! { cx,
            <div
                class="shared-input-container"
            >
                <input
                    id=id.clone()
                    class={class}
                    class:shared-input=true
                    class:textarea=true
                    class:shared-textarea=true
                    class:shared-input-invalid=
                        {move || {
                            if let Some(is_invalid) = is_invalid { is_invalid() } else { false };
                            if let Some(invalid_text) = invalid_text { !invalid_text().is_empty() } else { false }
                        }
                    }
                    type=input_type
                    prop:value=value
                    value=value
                    on:input=move |e| {
                        let target = event_target::<HtmlInputElement>(&e);
                        target.set_value(&value());
                        // if let Some(mask) = &mask {
                        //     input_mask(value, e, &mask.0, mask.1.as_deref())
                        // } else {
                        //     value.set(event_target_value(&e))
                        // };
                        // if let Some(is_invalid) = is_invalid {
                        //     is_invalid.set(false)
                        // }
                        // if let Some(invalid_text) = invalid_text {
                        //     invalid_text.update(|prev| prev.clear())

                        // }
                    }
                />
                <div />
                <label for=id class:filled-input=move || {!value().is_empty()}>
                    {move || {
                        if let Some(invalid_text) = invalid_text {
                            if !invalid_text().is_empty() {
                                return invalid_text()
                            }
                        }
                        label.clone()
                    }}
                </label>
                <div class="variants">
                    {move || {
                        {variants
                            .clone()
                            .into_iter()
                            .map(|n| view! { cx, 
                                <span>{n}</span>
                            })
                            .collect_view(cx)}
                    }}
                </div>
            </div>
        },
    }
}

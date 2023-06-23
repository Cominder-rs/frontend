use leptos::*;

#[component]
pub fn Cross(cx: Scope, #[prop(optional, into)] class: Option<AttributeValue>) -> impl IntoView {
    view! { cx,
        <svg width="13" height="13" viewBox="0 0 13 13" fill="none" xmlns="http://www.w3.org/2000/svg" class=class>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M0.488235 12.5118C-0.0651318 11.9584 -0.162652 11.1587 0.270418 10.7257L10.7256 0.270515C11.1587 -0.162552 11.9584 -0.0650318 12.5117 0.488332C13.0651 1.0417 13.1626 1.84135 12.7296 2.27442L2.27434 12.7296C1.84127 13.1627 1.0416 13.0651 0.488235 12.5118Z" fill="white"/>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M0.488252 0.488232C1.04162 -0.0651314 1.84128 -0.162651 2.27435 0.270416L12.7296 10.7256C13.1627 11.1586 13.0651 11.9583 12.5118 12.5117C11.9584 13.065 11.1587 13.1626 10.7257 12.7295L0.270434 2.27432C-0.162635 1.84126 -0.0651153 1.0416 0.488252 0.488232Z" fill="white"/>
        </svg>
    }
}

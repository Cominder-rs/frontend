use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::stores::notifier_store::{Notification, NotificationVariant, NotifierStore};

#[component]
pub fn Notifier(cx: Scope) -> impl IntoView {
    let NotifierStore { queue, .. } =
        use_context::<NotifierStore>(cx).expect("Getting NotifierStore");
    
    view! {cx ,
        <div class="notifications">
            <For
                each=queue
                key=|(notification, _)| notification.id
                view=move |cx, (notification, node_ref)| {
                    view! {cx,
                        <div _ref=node_ref
                            class:is-info=move ||notification.variant == NotificationVariant::Info
                            class:is-success=move ||notification.variant == NotificationVariant::Success
                            class:is-error=move ||notification.variant == NotificationVariant::Error
                        >
                            <div class="icon-container">
                                <i
                                    class="fa-solid fa-xl"
                                    class:fa-circle-info=move || notification.variant == NotificationVariant::Info
                                    class:fa-circle-check=move || notification.variant == NotificationVariant::Success
                                    class:fa-triangle-exclamation=move || notification.variant == NotificationVariant::Error
                                />
                            </div>
                            <div class="text-content">{notification.text_content}</div>
                        </div>
                    }
                }
            />
        </div>
    }
}

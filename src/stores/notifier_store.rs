use leptos::{html::Div, *};
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationVariant {
    Info,
    Success,
    Error,
}

#[derive(Clone)]
pub struct Notification {
    pub variant: NotificationVariant,
    pub text_content: String,
    pub id: i32,
    pub time_to_hide: i32,
}

#[derive(Clone, Copy)]
pub struct NotifierStore {
    pub queue: RwSignal<Vec<(Notification, NodeRef<Div>)>>,
    pub next_id: RwSignal<i32>,
}

impl NotifierStore {
    pub fn init(cx: Scope) -> Self {
        NotifierStore {
            queue: create_rw_signal(cx, vec![]),
            next_id: create_rw_signal(cx, 0),
        }
    }

    pub fn notify(
        &mut self,
        cx: Scope,
        variant: NotificationVariant,
        text_content: String,
        time_to_hide: Option<i32>,
    ) {
        let node_ref: NodeRef<Div> = create_node_ref(cx);
        let timer = if let Some(timer) = time_to_hide {
            timer
        } else {
            4000
        };
        self.next_id.update_untracked(|prev| *prev += 1);
        self.queue.update(|prev| {
            prev.push((
                Notification {
                    variant,
                    text_content,
                    id: self.next_id.get_untracked(),
                    time_to_hide: timer,
                },
                node_ref,
            ))
        });

        let queue = self.queue;
        let id = self.next_id.get_untracked();
        let callback = Closure::wrap(Box::new(move || {
            queue.update(move |notifications| {
                notifications.retain(|(notification, _)| notification.id != id)
            });
        }) as Box<dyn Fn()>);
        _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.into_js_value().unchecked_ref(),
            timer + 300,
        );

        let callback = Closure::wrap(Box::new(move || {
            let node = node_ref.get_untracked().unwrap();
            node.class("hide", true);
        }) as Box<dyn Fn()>);
        _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.into_js_value().unchecked_ref(),
            timer,
        );
    }
}

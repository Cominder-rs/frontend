use leptos::*;

#[derive(Clone, Copy)]
pub struct CounterStore {
    cx: Scope,
    pub counter: RwSignal<u32>,
    pub shit: RwSignal<String>,
}

impl CounterStore {
    pub fn init(cx: Scope) -> CounterStore {
        CounterStore {
            cx,
            counter: create_rw_signal(cx, 0),
            shit: create_rw_signal(cx, "Shit".to_string()),
        }
    }

    pub fn inc(&self) {
        self.counter.update(|count: &mut u32| *count += 1);
    }

    pub fn change_shit(&self) {
        self.shit.update(|shit| shit.push_str("loh"))
    }
}

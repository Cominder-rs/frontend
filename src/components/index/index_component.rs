use leptos::*;
use super::new_post::NewPost;

#[derive(Clone, Copy)]
enum IndexState {
    Main,
    Post,
    NewPost,
}

#[component]
pub fn Index(cx: Scope) -> impl IntoView {
    let index_state = create_rw_signal(cx, IndexState::NewPost);

    let on_circle = move |_| {
        index_state.set(IndexState::NewPost);    
    };
    
    view! { cx, 
        <div class="index_page">
            {move || match index_state() {
                IndexState::Main => unreachable!(),
                IndexState::Post => unreachable!(),
                IndexState::NewPost => view! {cx, <NewPost /> }
            }}
            <div class="circle" on:click=on_circle>
                <img src="./assets/star.svg" />
            </div>
        </div>
    }
}
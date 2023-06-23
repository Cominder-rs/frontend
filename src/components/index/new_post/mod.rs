use crate::{
    shared::{Input, InputVariant},
    svgs::Cross,
};
use leptos::*;
use posts_proto::Category;

#[component]
pub fn NewPost(cx: Scope) -> impl IntoView {
    let title = create_rw_signal(cx, String::new());
    let category = create_rw_signal(cx, String::new());
    let short_description = create_rw_signal(cx, String::new());
    let detailed_description = create_rw_signal(cx, String::new());
    let categories = vec![
        "IT",
        "Искусство",
        "Спорт",
        "Киберспорт",
        "Стартапы",
        "Танцы",
        "Музыка",
        "Волонтёрство",
    ]
        .iter()
        .map(|x| x.to_string())
        .collect();

    view! { cx,
        <div class="new-post">
            <div class="new-post-header">
                <p>"Разместить объявление"</p>
                <div><Cross /></div>
            </div>
            <div>
                <p>"Категория"</p>
                <Input
                    value=category
                    input_type="text".to_string()
                    id="category".to_string()
                    label="Категория".to_string()
                    variant=InputVariant::Picker(categories)
                />
            </div>
            <Input
                value=title
                input_type="text".to_string()
                id="title".to_string()
                label="Заголовок".to_string()
                variant=InputVariant::Classic
            />
            <Input
                value=short_description
                input_type="text".to_string()
                id="short-description".to_string()
                label="Краткое описание".to_string()
                variant=InputVariant::Textarea
            />
            <Input
                value=detailed_description
                input_type="text".to_string()
                id="detailed-description".to_string()
                label="Подробное описание (необязательно)".to_string()
                variant=InputVariant::Textarea
            />
        </div>
    }
}

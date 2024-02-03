use leptos::*;
use comrak::{markdown_to_html, Options};

#[component]
pub fn Markdown(
    #[prop(into, default = "".to_string())] markdown: String,
) -> impl IntoView {
    view! {
        <div inner_html={markdown_to_html(&markdown, &Options::default())}>
        </div>
    }
}
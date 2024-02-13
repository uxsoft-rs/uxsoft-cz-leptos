use leptos::*;
use comrak::{markdown_to_html, Options};

#[component]
pub fn Markdown(
    #[prop(into, default = "".to_string())] markdown: String,
) -> impl IntoView {
    let mut options = Options::default();
    options.extension.front_matter_delimiter = Some("---".to_string());

    let html = markdown_to_html(&markdown, &options); 
    view! {
        <div class="prose" inner_html={html}>
        </div>
    }
}
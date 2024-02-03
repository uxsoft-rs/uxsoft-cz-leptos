use crate::app::components::{card::Card, markdown::Markdown};
use leptos::*;

static WEB_DEV_MD: &str = include_str!(r"./01-web-dev.md");
static APP_DEV_MD: &str = include_str!(r"./02-app-dev.md");

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="grid grid-cols-2 gap-2 mt-4">
            <Card title="Web Development">
                <Markdown markdown={WEB_DEV_MD}/>
            </Card>
            <Card title="App Development">
                <Markdown markdown={APP_DEV_MD}/>
            </Card>
        </div>
    }
}

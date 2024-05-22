use crate::app::components::{card::Card, markdown::Markdown, page_title::PageTitle};
use leptos::*;

static WEB_DEV_MD: &str = include_str!(r"./01-web-dev.md");
static APP_DEV_MD: &str = include_str!(r"./02-app-dev.md");

#[component]
pub fn HomePage() -> impl IntoView {

    let alpaca = crate::app::routes::projects::PROJECTS.iter()
        .find(|u|u.id == "alpaca")
        .map(|p| p.short);

    let awk = crate::app::routes::projects::PROJECTS.iter()
        .find(|u|u.id == "awk")
        .map(|p| p.short);

    view! {
        <>
            <PageTitle title="Welcome"/>
            <div class="grid grid-cols-2 mt-4 gap-2">
                <Card title="Web Development">
                    <Markdown markdown={WEB_DEV_MD}/>
                </Card>
                <Card title="App Development">
                    <Markdown markdown={APP_DEV_MD}/>
                </Card>
                <Card title="Alpaca" image="/images/Projects/Alpaca/screenshot1.jpg" href="/projects/alpaca">
                    {alpaca}
                </Card>
                <Card title="AppleWirelessKeyboard" image="/images/Projects/AppleWirelessKeyboard/screen1-full.jpg" href="/projects/awk">
                    {awk}
                </Card>
            </div>
        </>
    }
}

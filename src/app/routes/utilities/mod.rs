use leptos::*;
use leptos_router::*;

use crate::app::components::markdown::Markdown;

pub struct Util<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub markdown: &'a str,
}

pub const UTILITIES: &'static [Util] = &[Util {
    id: "battery",
    name: "Battery Indicator",
    markdown: include_str!("./battery.mdx"),
}];

#[component]
pub fn UtilitiesPage() -> impl IntoView {
    view! {
        <div>
            "Utilities" <ul>{UTILITIES.iter().map(|u| view! { <li>{u.name}</li> }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn UtilityPage() -> impl IntoView {
    let params = use_params_map();

    let id = params
        .with(|params| params.get("id").cloned())
        .unwrap_or_default();

    if let Some(util) = UTILITIES.iter().find(|i| i.id == id) {
        view! { 
            <div>
                "Utility" 
                {id}
                {util.name}

                <Markdown markdown={util.markdown}/>
            </div> 
        }
    }
    else
    {
        view! {
            <div>
                "Page not found"
            </div>
        }
    }
}

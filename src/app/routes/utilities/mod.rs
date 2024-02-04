use std::collections::HashMap;

use gray_matter::{engine::YAML, Matter};
use leptos::*;
use leptos_router::*;
use rust_embed::{EmbeddedFile, RustEmbed};
use serde::Deserialize;

use crate::app::components::markdown::Markdown;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
}

#[derive(RustEmbed)]
#[folder = "src/app/routes/utilities/"]
#[include = "*.md"]
#[include = "*.mdx"]
#[exclude = "*.rs"]
struct Asset;

impl Asset {
    fn files() -> HashMap<String, (FrontMatter, String)> {
        Asset::iter()
            .map(|f| {
                let ef = Asset::get(f.as_ref()).unwrap();
                let contents = std::str::from_utf8(ef.data.as_ref()).unwrap();

                let matter = Matter::<YAML>::new();
                let result = matter.parse(contents);
                let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();

                (f.to_string(), (front_matter, contents.to_string()))
            })
            .collect()
    }
}

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
            "Utilities" // <ul>
            // {Asset::iter().map(|f| view! { <li>{f.to_string()}</li> }).collect_view()}
            // </ul>
            <ul>
                {Asset::files()
                    .iter()
                    .map(|(n, (f, c))| view! { <li>{&f.title} <br/> {c}</li> })
                    .collect_view()
                }
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
        view! { <div>"Utility" {id} {util.name} <Markdown markdown=util.markdown/></div> }
    } else {
        view! { <div>"Page not found"</div> }
    }
}

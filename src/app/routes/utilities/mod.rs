use std::collections::HashMap;

use crate::app::components::markdown::Markdown;
use crate::app::components::page_title::PageTitle;
use gray_matter::{engine::YAML, Matter};
use leptos::*;
use leptos_router::*;
use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    downloadUrl: String,
    imageUrl: String,
    icon: String,
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

                let (id, _) = f.split_once('.').unwrap();

                (id.to_string(), (front_matter, contents.to_string()))
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
            // <ul>
            "Utilities"
            // {Asset::iter().map(|f| view! { <li>{f.to_string()}</li> }).collect_view()}
            // </ul>
            <ul>
                {Asset::files()
                    .iter()
                    .map(|(n, (f, c))| view! { <li>{n} <br/> {&f.title} <br/> {c}</li> })
                    .collect_view()}

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

    let assets = Asset::files();
    if let Some((frontmatter, content)) = assets.get(&id) {
        view! {
            <>
                <PageTitle title={&frontmatter.title}/>
                <div class="text-center mx-0 my-8">
                    <img
                        src=&frontmatter.imageUrl
                        alt="Screenshot"
                        title=&frontmatter.title
                        class="border-0 mx-auto"
                    />
                </div>
                <article class="prose max-w-full">
                    <Markdown markdown=content/>
                </article>
                <div class="h-8 mt-4">
                    <a
                        href=&frontmatter.downloadUrl
                        class="btn btn-sm btn-primary float-right"
                        type="primary"
                    >
                        Download
                    </a>
                </div>
            </>
        }
    } else {
        view! { <>"Page not found"</> }
    }
}

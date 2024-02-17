use crate::app::components::markdown::Markdown;
use crate::app::components::page_title::PageTitle;
use leptos::*;
use leptos_router::*;

pub struct Post {
    pub id: &'static str,
    pub title: &'static str,
    pub markdown: &'static str,
}

pub const POSTS: &'static [&'static Post] = &[
    &Post {
        id: "hiring-behavioral-questions",
        title: "Hiring with Behavioral Questions",
        markdown: include_str!("hiring-behavioral-questions.md"),
    },
    &Post {
        id: "hiring-developer-questions",
        title: "Hiring a Developer",
        markdown: include_str!("hiring-developer-questions.md"),
    },
    &Post {
        id: "kubernetes-on-pi",
        title: "Kubernetes on Raspberry Pi",
        markdown: include_str!("kubernetes-on-pi.md"),
    }
];

#[component]
pub fn ListItem(
    #[prop(into, default = "".to_string())] title: String,
    #[prop(into, default = "".to_string())] url: String,
    #[prop(into, default = None)] icon: Option<String>,
) -> impl IntoView {
    view! {
        <li>
            <a
                href=url
                class="rounded p-2 cursor-pointer flex items-center gap-2 transition-colors duration-500 ease-in-out hover:bg-orange-100"
            >
                {match icon {
                    None => {
                        view! {
                            <>
                                <span>{title}</span>
                            </>
                        }
                    }
                    Some(icon) => {
                        view! {
                            <>
                                <img
                                    src=icon
                                    alt=title
                                    height=48
                                    width=48
                                    class="object-cover rounded w-[48px] h-[48px]"
                                />
                            </>
                        }
                    }
                }}
            </a>
        </li>
    }
}

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <div>
            <PageTitle title="Utilities"/>

            <ul>
                {POSTS
                    .iter()
                    .map(|u| {
                        view! { <ListItem title={u.title} url=format!("/blog/{}", u.id)/> }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();

    let component = move || {
        params.with(|params| params
            .get("id")
            .cloned()
            .and_then(|id|POSTS.iter().find(|u|u.id==id)))
            .map(|post| view! {
                <>
                    <PageTitle title=post.title/>
                    <Markdown markdown=post.markdown/>
                </>
            })
            .unwrap_or(view! {
                <>"Page not found"</>
            })
    };

    view! {
        <>
            {component}
        </>
    }
}

use crate::app::components::markdown::Markdown;
use crate::app::components::page_title::PageTitle;
use leptos::*;
use leptos_router::*;

pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub download_url: &'static str,
    pub donate_button_id: &'static str,
    pub thumbnail_url: &'static str,
    pub image_url: &'static str,
    pub markdown: &'static str,
}

impl Project {
    fn images(&self) -> Vec<String> {
        (1..4)
            .map(|i| self.image_url.replace("{0}", i.to_string().as_str()))
            .collect()
    }

    fn thumbnails(&self) -> Vec<String> {
        (1..4)
            .map(|i| self.thumbnail_url.replace("{0}", i.to_string().as_str()))
            .collect()
    }
}

pub const PROJECTS: &'static [&'static Project] = &[
    &Project {
        id: "alpaca",
        title: "Alpaca",
        download_url: "https://alpaca.uxsoft.cz",
        donate_button_id: "4RVXP9XXVGBPA",
        thumbnail_url: "/images/Projects/Alpaca/screenshot{0}.jpg",
        image_url: "/images/Projects/Alpaca/screenshot{0}.jpg",
        markdown: include_str!("alpaca.md"),
    },
    &Project {
        id: "appleholik",
        title: "Appleholik",
        download_url: "/images/Utilities/CantLockThis/cantlockthis.exe",
        donate_button_id: "",
        thumbnail_url: "",
        image_url: "/images/Utilities/CantLockThis/cantlockthis.png",
        markdown: include_str!("appleholik.md"),
    },
    &Project {
        id: "awk",
        title: "AppleWirelessKeyboard",
        download_url: "/images/Utilities/ChromeHistoryDisabler/chromehistorydisabler.exe",
        donate_button_id: "",
        thumbnail_url: "",
        image_url: "/images/Utilities/ChromeHistoryDisabler/chromehistorydisabler.jpg",
        markdown: include_str!("awk.md"),
    },
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
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <div>
            <PageTitle title="Projects"/>

            <ul>
                {PROJECTS
                    .iter()
                    .map(|u| {
                        view! { <ListItem title={u.title} url=format!("/projects/{}", u.id)/> }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();

    let component = move || {
        params.with(|params| params
            .get("id")
            .cloned()
            .and_then(|id|PROJECTS.iter().find(|u|u.id==id)))
            .map(|project| view! {
                <>
                    <PageTitle title=project.title/>
                
                    <div class="carousel rounded-box">
                        {
                            project.thumbnails().iter().map(|i| view! {
                                <div class="carousel-item">
                                    <a>
                                        <img src={i} alt="app screenshot" class="w-[900px]" />
                                    </a>
                                </div>
                            }).collect_view()
                        }
                    </div>
                
                    <article class="prose">
                        <Markdown markdown={project.markdown}/>
                    </article>
                
                    <div class="text-right mx-3 mt-6">
                        <Show when={move || project.donate_button_id.len() > 0}>
                            <form
                                class="inline-block mr-3"
                                action="https://www.paypal.com/cgi-bin/webscr"
                                method="post"
                                target="_top">
                                <input type="hidden" name="cmd" value="_s-xclick"/>
                                <input type="hidden" name="hosted_button_id" value={project.donate_button_id}/>
                                <button class="btn btn-sm" type="submit">Donate</button>
                            </form>
                        </Show>
                        <Show when={move || project.download_url.len() > 0}>
                            <a class="btn btn-primary btn-sm" href={project.download_url}>Download</a>
                        </Show>
                    </div>
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

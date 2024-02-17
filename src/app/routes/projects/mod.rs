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
    pub short: &'static str,
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
        short: "Alpaca is an all-in-one product development tool aimed at organisations applying agile at scale. 
It's an opinionated tool with built-in best practices from industry experts.
The tool will softly guide you through the product development process end-to-end.
Using Alpaca will make your work fun."
    },
    &Project {
        id: "appleholik",
        title: "Appleholik",
        download_url: "https://appleholik.uxsoft.cz",
        donate_button_id: "",
        thumbnail_url: "/images/Projects/Appleholic/screenshot{0}.jpg",
        image_url: "/images/Projects/Appleholic/screenshot{0}.jpg",
        markdown: include_str!("appleholik.md"),
        short: "Appleholic is a very simplistic news aggregator completely focused on Czech Apple news.
It features each source in a vertical list and includes a short.",
    },
    &Project {
        id: "awk",
        title: "AppleWirelessKeyboard",
        download_url: "https://install.appcenter.ms/users/uxsoft/apps/applewirelesskeyboard/distribution_groups/public",
        donate_button_id: "4RVXP9XXVGBPA",
        thumbnail_url: "/images/Projects/AppleWirelessKeyboard/screen{0}-full.jpg",
        image_url: "/images/Projects/AppleWirelessKeyboard/screen{0}-full.jpg",
        markdown: include_str!("awk.md"),
        short: "Do you love Apple Wireless Keyboard but also love Microsoft Windows? Ever wanted to use them together?
Now you can without losing your media functionality.
This Windows application brings your Fn key and media keys back to life with Mac-like status overlays.
It lets you choose if you want to access media keys with Fn or instead of Function keys, by just pressing the eject key.",
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
                
                    <Markdown markdown={project.markdown}/>
                
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

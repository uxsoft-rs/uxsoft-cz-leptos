use crate::app::components::list_item::ListItem;
use crate::app::components::markdown::Markdown;
use crate::app::components::page_title::PageTitle;
use crate::app::components::carousel::Carousel;
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
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <div>
            <PageTitle title="Projects"/>

            <ul>
                {PROJECTS
                    .iter()
                    .map(|p| {
                        let icon_url = p.thumbnail_url.replace("{0}", "1");
                        view! { <ListItem title={p.title} url={format!("/projects/{}", p.id)} icon={icon_url}/> }
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
            .and_then(|id| PROJECTS.iter().find(|u|u.id == id)))
            .map(|project| view! {
                <>
                    <PageTitle title=project.title/>

                    <Carousel images={project.thumbnails()}/>
                
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

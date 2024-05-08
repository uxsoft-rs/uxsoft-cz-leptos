use crate::app::components::markdown::Markdown;
use crate::app::components::page_title::PageTitle;
use leptos::*;
use leptos_router::*;

pub struct Util {
    pub id: &'static str,
    pub title: &'static str,
    pub download_url: &'static str,
    pub image_url: &'static str,
    pub markdown: &'static str,
}

pub const UTILS: &'static [&'static Util] = &[
    &Util {
        id: "battery",
        title: "uxsoft.Battery",
        download_url: "/images/Utilities/uxsoft.Battery/uxsoftbattery.exe",
        image_url: "/images/Utilities/uxsoft.Battery/screen1-thumb.jpg",
        markdown: include_str!("battery.md"),
    },
    &Util {
        id: "cantlockthis",
        title: "Can't Lock This",
        download_url: "/images/Utilities/CantLockThis/cantlockthis.exe",
        image_url: "/images/Utilities/CantLockThis/cantlockthis.png",
        markdown: include_str!("cant-lock-this.md"),
    },
    &Util {
        id: "chromehistorydisabler",
        title: "Chrome History is History",
        download_url: "/images/Utilities/ChromeHistoryDisabler/chromehistorydisabler.exe",
        image_url: "/images/Utilities/ChromeHistoryDisabler/chromehistorydisabler.jpg",
        markdown: include_str!("chrome-history-disabler.md"),
    },
    &Util {
        id: "listmaker",
        title: "List Maker",
        download_url: "/images/Utilities/ListMaker/listmaker.exe",
        image_url: "/images/Utilities/ListMaker/listmaker.jpg",
        markdown: include_str!("list-maker.md"),
    },
    &Util {
        id: "minesweeper",
        title: "Minesweeper",
        download_url: "/images/Utilities/Minesweeper/minesweeper.zip",
        image_url: "/images/Utilities/Minesweeper/minesweeper.PNG",
        markdown: include_str!("minesweeper.md"),
    },
    &Util {
        id: "passwordexpirynotifier",
        title: "Password Expiration Notifier",
        download_url: "/images/Utilities/PasswordExpirationNotifier/passwordexpirynotifier.exe",
        image_url: "/images/Utilities/PasswordExpirationNotifier/passwordexpirynotifier.jpg",
        markdown: include_str!("password-expiration-notifier.md"),
    },
    &Util {
        id: "secondmonitortime",
        title: "Second Monitor Time",
        download_url: "/images/Utilities/SecondMonitorTime/SecondMonitorTime.exe",
        image_url: "/images/Utilities/SecondMonitorTime/secondmonitortime.png",
        markdown: include_str!("second-monitor-time.md"),
    },
    &Util {
        id: "wordprotectionremover",
        title: "MS Word Protection Remover",
        download_url: "/images/Utilities/WordProtectionRemover/wordprotectionremover.exe",
        image_url: "/images/Utilities/WordProtectionRemover/wordprotectionremover.jpg",
        markdown: include_str!("word-protection-remover.md"),
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
                {
                    if let Some(icon) = icon {
                        view! {
                            <>
                                <img
                                    src=icon
                                    alt=title.clone()
                                    height=48
                                    width=48
                                    class="object-cover rounded w-[48px] h-[48px]"
                                />
                            </>
                        }
                    } else { 
                        view! { <>()</> } 
                    }
                }

                <span>{title}</span>
            </a>
        </li>
    }
}

#[component]
pub fn UtilitiesPage() -> impl IntoView {
    view! {
        <div>
            <PageTitle title="Utilities"/>

            <ul>
                {UTILS
                    .iter()
                    .map(|u| {
                        view! {
                            <ListItem
                                title=u.title
                                url=format!("/utilities/{}", u.id)
                                icon=Some(u.image_url.to_string())
                            />
                        }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}

#[component]
pub fn UtilityPage() -> impl IntoView {
    let params = use_params_map();

    let component = move || {
        params
            .with(|params| {
                params
                    .get("id")
                    .cloned()
                    .and_then(|id| UTILS.iter().find(|u| u.id == id))
            })
            .map(|util| {
                view! {
                    <>
                        <PageTitle title=util.title/>
                        <div class="text-center mx-0 my-8">
                            <img
                                src=util.image_url
                                alt="Screenshot"
                                title=util.title
                                class="border-0 mx-auto"
                            />
                        </div>
                        <Markdown markdown=util.markdown/>
                        <div class="h-8 mt-4">
                            <a
                                href=util.download_url
                                class="btn btn-sm btn-primary float-right"
                                rel="external"
                                download
                            >
                                Download
                            </a>
                        </div>
                    </>
                }
            })
            .unwrap_or(view! { <>"Page not found"</> })
    };

    view! { <>{component}</> }
}

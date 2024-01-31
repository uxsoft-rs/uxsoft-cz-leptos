use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod menu;
use menu::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uxsoft-cz-leptos.css"/>

        <Title text="uxsoft"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    // <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SubMenu(
    #[prop(into, default = "".to_string())] title: String,
    #[prop(into, default = "".to_string())] href: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li class="
            leading-8 no-underline text-black font-bold text-sm border-t-[5px] border-solid border-gray-300 
            hover:bg-white hover:border-orange-400">
            <a href=href>{title}</a>
            <ul>
                {
                    if let Some(children) = children {
                        children().into_view()
                    } else {
                        ().into_view()
                    }
                }
            </ul>
        </li>
    }
}

#[component]
fn MenuItem(
    #[prop(into, default = "".to_string())] title: String,
    #[prop(into, default = "".to_string())] href: String,
) -> impl IntoView {
    view! {
        <li class="">
            <a href={href}>{title}</a>
        </li>
    }
}

#[component]
fn Menu(children: Children) -> impl IntoView {
    view! { <ul class="grid grid-cols-4 cursor-pointer font-bold">{children()}</ul> }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="bg-gray-200">
            <Menu>
                <SubMenu title="Projects" href="/projects">
                    <MenuItem title="Apple Wireless Keyboard" href="/projects/awk" />
                    <MenuItem title="Alpaca" href="/projects/alpaca" />
                    <MenuItem title="Wishlist" href="/projects/awk" />
                </SubMenu>
                <SubMenu title="Utilities">
                </SubMenu>
                <SubMenu title="Blog">
                </SubMenu>
                <SubMenu title="About">
                </SubMenu>
            </Menu>
            <h1>"Welcome to Leptos!"</h1>
            <p>Is this hot reload working?</p>
            <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    // #[cfg(feature = "ssr")]
    // {
    //     // this can be done inline because it's synchronous
    //     // if it were async, we'd use a server function
    //     let resp = expect_context::<leptos_axum::ResponseOptions>();
    //     resp.set_status(axum::http::StatusCode::NOT_FOUND);
    // }

    view! { <h1>"Not Found"</h1> }
}

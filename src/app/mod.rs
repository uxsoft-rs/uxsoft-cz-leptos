use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod footer;
mod header;
mod routes;
pub mod components;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uxsoft-cz-leptos.css"/>

        <Title text="uxsoft"/>
        <Router>
            <div id="page" class="mx-auto w-[900px]">
                <header::Header/>

                <main>
                    <Routes>
                        <Route path="" view=routes::home::HomePage/>
                        <Route path="/about" view=routes::about::AboutPage/>
                        <Route path="/projects" view=routes::projects::ProjectsPage/>
                        <Route path="/projects/:id" view=routes::projects::ProjectPage/>
                        <Route path="/blog" view=routes::blog::BlogPage/>
                        <Route path="/blog/:id" view=routes::blog::BlogPostPage/>
                        <Route path="/utilities" view=routes::utilities::UtilitiesPage/>
                        <Route path="/utilities/:id" view=routes::utilities::UtilityPage/>
                    </Routes>
                </main>

                <footer::Footer/>
            </div>
        </Router>
    }
}

#[component]
fn NotImplementedYetPage() -> impl IntoView {
    view! {
        <div>
            "Not implemented yet"
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

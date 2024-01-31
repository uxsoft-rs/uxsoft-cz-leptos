use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod footer;
mod header;
mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uxsoft-cz-leptos.css"/>

        <Title text="uxsoft"/>
        <Router>
            <div id="page" class="mx-auto w-[900px]">
                <header::Header></header::Header>

                <main>
                    <Routes>
                        <Route path="" view=routes::home::HomePage/>
                        <Route path="/about" view=routes::about::AboutPage/>
                    </Routes>
                </main>

                <footer::Footer></footer::Footer>
            </div>
        </Router>
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

use crate::app::{components::page_title::PageTitle, routes::blog::article::Article};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let posts = expect_context::<Resource<(), Result<Vec<Article>, ServerFnError>>>();

    let post = Resource::new(
        move || params.get().get("id").cloned(),
        move |id| async move {
            match id {
                None => None,
                Some(id) => posts
                    .get()
                    .and_then(|r| r.ok())
                    .and_then(|posts| posts.into_iter().find(|p| p.id == id)),
            }
        },
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            { move || match post.get() {
                    None => view! { "Loading..." }.into_view(),
                    Some(None) => view! { "Error..." }.into_view(),
                    Some(Some(post)) => view! {
                        <PageTitle title=post.metadata.title/>
                        <article class="prose max-w-full" inner_html=post.content>

                        </article>
                    }.into_view()
                }

            }
        </Suspense>
    }
}

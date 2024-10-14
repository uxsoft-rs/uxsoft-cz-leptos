use crate::app::components::list_item::ListItem;
use crate::app::components::page_title::PageTitle;
use leptos::*;
use leptos_router::*;

pub mod server;
pub mod article;

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = expect_context::<Resource<(), Result<Vec<article::Article>, ServerFnError>>>();

    view! {
        <div>
            <PageTitle title="Blog"/>
            <Suspense>
            { move || match posts.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(Err(e)) => view! { <p>{move || format!("Error loading posts: {:?}", e)}</p> }.into_view(),
                    Some(Ok(data)) => view!{
                        <ul>
                            {data
                                .into_iter()
                                .map(|post| view!{
                                    <ListItem title={post.metadata.title} url=format!("/blog/{}", post.id)/>
                                })
                                .collect_view()
                            }
                        </ul>
                    }.into_view()
                }
            }
            </Suspense>
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let posts = expect_context::<Resource<(), Result<Vec<article::Article>, ServerFnError>>>();

    let post = Resource::new(
        move || params.get().get("id").cloned(), 
        move |id| async move {
            match id {
                None => None,
                Some(id) => posts.get()
                    .and_then(|r| r.ok())
                    .and_then(|posts| posts.into_iter().find(|p| p.id == id))
            }
        }
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            { move || match post.get() {
                    None => view! { "Loading..." }.into_view(),
                    Some(None) => view! { "Error..." }.into_view(),
                    Some(Some(post)) => view! {
                        <PageTitle title=post.metadata.title/>
                        <article class="prose" inner_html=post.content>
                            
                        </article>
                    }.into_view()
                }

            }
        </Suspense>
    }
}

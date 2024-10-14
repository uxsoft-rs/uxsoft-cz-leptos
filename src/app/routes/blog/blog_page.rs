use leptos::*;

use crate::app::{components::{list_item::ListItem, page_title::PageTitle}, routes::blog::article::Article};

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = expect_context::<Resource<(), Result<Vec<Article>, ServerFnError>>>();

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

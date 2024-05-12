use leptos::*;
use leptos_image::Image;

#[component]
pub fn Card(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] image: MaybeSignal<String>,
    #[prop(optional, into)] href: MaybeSignal<String>,
    children: ChildrenFn
) -> impl IntoView {
    view! {
        <div class="card bg-base-100"> // shadow-xl
            { move || 
                if image.get().len() > 0 { 
                    view! {
                        <figure>
                            <Image
                                class="w-[446px] h-[259px]"
                                src={image.get()}
                                alt="Card image"
                                width=892
                                height=518
                                quality=85
                                blur=true />
                        </figure>
                    }.into_view() 
                } else { 
                    "".into_view() 
                }
            }
            <div class="card-body">
                <h2 class="card-title">{title()}</h2>
                {children()}
                {move || if !href.get().is_empty() {
                    view! { 
                        <div class="card-actions justify-end">
                            <a href=href.get() class="btn btn-primary btn-sm">
                                Learn More
                            </a>
                        </div>
                     }.into_view()
                } else { view!{""}.into_view()}}

            </div>
        </div>
    }
}
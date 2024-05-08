use leptos::*;
use leptos_router::A;

#[component]
pub fn Carousel(
    #[prop(into, default = vec![])] images: Vec<String>,
) -> impl IntoView {
    view! {
        <div class="carousel w-full">
            {images.iter().enumerate().map(|(i, img)| {
                let this_id = format!("slide{}", i);
                let next_href = format!("#slide{}", (i as i32 + 1).rem_euclid(images.len() as i32));
                let prev_href = format!("#slide{}", (i as i32 - 1).rem_euclid(images.len() as i32));
                return view!{ 
                    <div id=this_id class="carousel-item relative w-full">
                        <img src=img class="w-full object-contain" />
                        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                            <A href=prev_href class="btn btn-circle">"❮"</A> 
                            <A href=next_href class="btn btn-circle">"❯"</A>
                        </div>
                    </div> 
                    }
                }).collect_view()}
        </div>
    }
}
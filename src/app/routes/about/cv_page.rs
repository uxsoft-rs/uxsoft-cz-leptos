use leptos::*;

#[component]
pub fn CvPage(children: Children) -> impl IntoView {
    view! { 
        <div class="border-neutral-200 mx-auto pt-6 px-5">
            {children()}
        </div> 
    }
}
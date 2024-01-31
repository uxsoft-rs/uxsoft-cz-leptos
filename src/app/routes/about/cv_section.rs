use leptos::*;

#[component]
pub fn CvSection(
    #[prop(into, default = "".to_string())] title: String,
    children: Children
) -> impl IntoView {
    view! { 
        <div class="my-4">
            <h3 class="text-neutral-500 underline font-normal text-sm leading-5 my-4;">{title}</h3>
            {children()}
        </div> 
    }
}
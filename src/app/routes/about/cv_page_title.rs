use leptos::*;

#[component]
pub fn CvPageTitle(
    #[prop(into, default = "".to_string())] title: String,
    children: Children
) -> impl IntoView {
    view! { 
        <div>
            <h1 class="font-medium text-base">{title}</h1>
            <p class="text-neutral-500 text-sm leading-5 mt-1">
                {children()}
            </p>
        </div> 
    }
}
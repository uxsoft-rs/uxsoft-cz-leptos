use leptos::*;

#[component]
pub fn CvButton(
    #[prop(into, default = "".to_string())] href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a href={href} class="no-underline text-inherit">
            <div class="duration-200 transition-colors ease-in-out text-sm leading-5 bg-neutral-200 cursor-pointer inline-block px-3 py-[3px] rounded-[3px] hover:bg-neutral-100 active:bg-neutral-400">
                {children()}
            </div>
        </a>
    }
}

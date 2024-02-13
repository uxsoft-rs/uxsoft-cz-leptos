use leptos::*;

#[component]
pub fn PageTitle(
    #[prop(into, default = "".to_string())] title: String,
) -> impl IntoView {
    view! {
        <h1 class="bg-orange-100 text-[22px] text-zinc-600 mx-0 mb-2 mt-6 pt-3 pb-2 px-5 border-x-8 border-solid border-x-orange-400 font-bold">
            {title}
        </h1>
    }
}
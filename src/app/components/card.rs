use leptos::*;

#[component]
pub fn Card(
    #[prop(into, default = "".to_string())] title: String,
    children: ChildrenFn
) -> impl IntoView {
    view! {
        <div class="m-2 p-2">
            <h1 class="text-lg font-bold">{title}</h1>
            {children()}
        </div>
    }
}
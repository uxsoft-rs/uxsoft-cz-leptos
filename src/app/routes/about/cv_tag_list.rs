use leptos::*;

#[component]
pub fn CvTagList(
    #[prop(into, default = "".to_string())] title: String,
    #[prop(default = vec![])] tags: Vec<String>,
) -> impl IntoView {
    view! {
        <div class="gap-3 flex">
            <div class="text-sm leading-5 min-w-[100px] py-[3px]">
                <p class="m-0">{title}</p>
            </div>
            <div class="gap-2 flex-wrap flex">
                {tags
                    .into_iter()
                    .map(|t| {
                        view! {
                            <div class="text-sm leading-5 text-center bg-neutral-200 px-3 py-[3px] rounded-[3px]">
                                <p class="text-sm leading-5 m-0 p-0">{t}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

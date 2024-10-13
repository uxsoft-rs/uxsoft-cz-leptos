use leptos::*;

#[component]
pub fn CvJob(
    #[prop(into, default = "".to_string())] title: String,
    #[prop(into, default = "".to_string())] start: String,
    #[prop(into, default = "Present".to_string())] end: String,
    #[prop(into, default = "".to_string())] company: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="text-neutral-500 w-full flex relative">
        <div class="gap-0 flex-col text-neutral-400 text-sm leading-5 min-w-[180px] flex mb-0">
            <p class="gap-0 m-0">{start} <span class="mx-1">-</span></p>
            <p class="gap-0 m-0">{end}</p>
        </div>
        <div class="gap-1 grid relative">
            <div class="gap-2 flex">
                <h2 class="font-bold text-[15px] inline-block m-0">
                    {title}
                </h2>
                <span class="text-neutral-400 text-sm leading-5 mt-px">
                    @{company}
                </span>
            </div>
            <div class="text-neutral-500 text-sm leading-5">
                {children()}
            </div>
        </div>
    </div>
    }
}

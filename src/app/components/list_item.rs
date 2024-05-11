use leptos::*;

#[component]
pub fn ListItem(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] url: MaybeSignal<String>,
    #[prop(optional, into)] icon: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <li>
            <a
                href=url
                class="rounded p-2 cursor-pointer flex items-center gap-2 transition-colors duration-500 ease-in-out hover:bg-orange-100"
            >
                {if !icon.get().is_empty() {
                    view! {
                            <img
                                src={icon.get()}
                                alt={title.get()}
                                height=48
                                width=48
                                class="object-cover rounded w-[48px] h-[48px]"
                            />
                    }.into_view()
                } else {
                        view! { "" }.into_view()
                }}

                <span>{title.get()}</span>
            </a>
        </li>
    }
}
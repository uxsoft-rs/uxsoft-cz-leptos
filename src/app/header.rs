use leptos::*;
use leptos_router::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="mb-3 pt-6 flex">
            <a rel="prefetch" href="/" class="text-transparent flex-none">
                <img id="logo" class="w-[37px] h-[37px]" src="/images/Images/logo.svg" alt="Logo"/>
            </a>
            <ul class="ux-menu flex-auto w-full flex font-bold text-xs cursor-pointer ml-2.5 m-0 p-0">
                <li>
                    <a rel="prefetch" href="/projects">
                        Projects
                    </a>
                    <ul>
                        <li>
                            <a href="https://alpaca.uxsoft.cz">Alpaca</a>
                        </li>
                        <li>
                            <a href="/projects/awk">Apple Wireless Keyboard</a>
                        </li>
                        <li>
                            <a href="https://apple.slickreader.eu/">Appleholik</a>
                        </li>
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/utilities">
                        Utilities
                    </a>
                    <ul>
                        {crate::app::routes::utilities::UTILS
                            .iter()
                            .map(|u| {
                                view! {
                                    <li>
                                        <A href=format!("/utilities/{}", u.id)>{u.title}</A>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/blog">
                        Blog
                    </a>
                    <ul>// {
                    // allBlogs
                    // .filter((item) => !item.frontmatter.draft)
                    // .map((item) => (
                    // <li>
                    // <a rel="prefetch" href={item.url}>
                    // {item.frontmatter.title}
                    // </a>
                    // </li>
                    // ))
                    // }
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/about">
                        About
                    </a>
                </li>
            </ul>
        </nav>
    }
}

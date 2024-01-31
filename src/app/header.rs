use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="mb-3 pt-6">
            <a rel="prefetch" href="/" class="text-transparent float-left">
                <img id="logo" class="w-[37px] h-[37px]" src="/images/Images/logo.svg" alt="Logo" />
            </a>
            <ul class="ux-menu float-right">
                <li>
                    <a rel="prefetch" href="/projects">Projects</a>
                    <ul>
                        // {
                        //     allProjects.map((item) => (
                        //         <li>
                        //             <a rel="prefetch" href={item.url}>
                        //                 {item.frontmatter.title}
                        //             </a>
                        //         </li>
                        //     ))
                        // }
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/utilities">Utilities</a>
                    <ul>
                        // {
                        //     allUtilities.map((item) => (
                        //         <li>
                        //             <a rel="prefetch" href={item.url}>
                        //                 {item.frontmatter.title}
                        //             </a>
                        //         </li>
                        //     ))
                        // }
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/blog">Blog</a>
                    <ul>
                        // {
                        //     allBlogs
                        //         .filter((item) => !item.frontmatter.draft)
                        //         .map((item) => (
                        //             <li>
                        //                 <a rel="prefetch" href={item.url}>
                        //                     {item.frontmatter.title}
                        //                 </a>
                        //             </li>
                        //         ))
                        // }
                    </ul>
                </li>
                <li>
                    <a rel="prefetch" href="/about">About</a>
                </li>
            </ul>
        </nav>
    }
}

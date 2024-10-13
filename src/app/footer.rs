use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="text-[13px] text-[#9e9ea6] text-right mt-4 border-t-[#e8e8e8] border-t border-solid print:hidden">
            <div class="w-[900px] text-right mx-auto my-0 py-2.5">
                by <a class="text-inherit no-underline ml-1" href="http://uxsoft.cz">uxsoft.cz</a>
            </div>
        </footer>
    }
}

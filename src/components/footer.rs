use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="text-gray-400 mt-8 bg-gray-800 py-4">
            <div class="container mx-auto text-center">
                <p class="text-[#CED4DA]">
                    Made with <span class="text-[#CED4DA]">"❤️"</span> using
                    <a href="#" class="text-blue-500 hover:underline mx-1 animate-color-change">Rust</a>,
                    <a href="#" class="text-blue-500 hover:underline mx-1 animate-color-change">Leptos</a> &
                    <a href="#" class="text-blue-500 hover:underline mx-1 animate-color-change">Preline</a>.
                </p>
                <span class="text-[#CED4DA]">"© Itehax. 2024"</span>
                <p><a href="#" class="text-blue-500 hover:underline animate-color-change">Rss</a></p>
            </div>
        </footer>
    }
}

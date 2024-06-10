use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! { 
        <nav class="bg-gray-800 px-4 py-3 shadow-md fixed w-full top-0 z-10">
            <div class="container mx-auto flex justify-between items-center">
                <a href="#" class="text-2xl font-bold text-white hover:text-gray-400 animate-color-change">
                    Pillar Bolg <span class="text-[#CED4DA] text-font-1">"💻🔴📚"</span>
                </a>
                <div class="flex space-x-4">
                    <a href="#about" class="text-gray-400 hover:text-gray-200">About</a>
                    <a href="#projects" class="text-gray-400 hover:text-gray-200">Projects</a>
                    <a href="#blog" class="text-gray-400 hover:text-gray-200">Blog</a>
                    <a href="#books" class="text-gray-400 hover:text-gray-200">Books</a>
                    <a href="#hire" class="text-gray-400 hover:text-gray-200">Hire Me</a>
                </div>
            </div>
        </nav>
    }
}

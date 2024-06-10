use leptos::{component, view, IntoView};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-800 to-black text-white">
            <div class="text-center">
                <h1 class="text-6xl font-bold">Oops!</h1>
                <p class="text-4xl mt-2">Page not found.</p>
                <p class="mt-4">
                    Sorry, we can not find the page you are looking for.
                </p>
                <a href="/" class="mt-6 inline-block bg-purple-500 text-white py-2 px-4 rounded hover:bg-purple-700">
                    Go Home
                </a>
            </div>
        </div>
    }
}

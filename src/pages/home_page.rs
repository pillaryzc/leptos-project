use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="flex items-center justify-center min-h-screen bg-gray-900 text-white">
            <div class="text-center mt-24">
                <h1 class="text-5xl font-bold text-white mb-4" style="background: linear-gradient(to right, #00C6FF, #0072FF); -webkit-background-clip: text; color: transparent;">
                    Pillar SS.
                </h1>
                <p class="text-xl text-gray-400 mb-4">
                    CS student interested in rust , Golang , Whatever stimulates my Brain.
                </p>
                <div class="flex justify-center space-x-4 text-blue-800-400 mb-6">
                    <a href="#" class="hover:text-blue-200"><i class="fab fa-github"></i></a>
                    <a href="#" class="hover:text-blue-200"><i class="fab fa-linkedin"></i></a>
                    <a href="#" class="hover:text-blue-200"><i class="fab fa-discord"></i></a>
                    <a href="#" class="hover:text-blue-200"><i class="fab fa-youtube"></i></a>
                    <a href="#" class="hover:text-blue-200"><i class="fab fa-twitch"></i></a>
                </div>
            </div>
        </section>
    }
}
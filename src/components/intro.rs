use leptos::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! { 
        <div class="text-center mt-16 mb-16">
            <img src="https://via.placeholder.com/150" class="w-32 h-32 rounded-full mx-auto mb-4" alt="Profile Picture" />
            <h1 class="text-5xl font-bold">"Hi there, I'm Khue 👋"</h1>
            <p class="text-xl text-gray-600 mt-4">"Welcome to my website, where I write about Linux, DevOps, homelab, workflow optimization, and more!"</p>
            <div class="flex justify-center space-x-4 mt-4">
                <a href="mailto:example@example.com" class="text-gray-600 hover:text-gray-900"><i class="fas fa-envelope"></i></a>
                <a href="https://github.com" class="text-gray-600 hover:text-gray-900"><i class="fab fa-github"></i></a>
                <a href="https://linkedin.com" class="text-gray-600 hover:text-gray-900"><i class="fab fa-linkedin"></i></a>
                <a href="https://twitter.com" class="text-gray-600 hover:text-gray-900"><i class="fab fa-twitter"></i></a>
                <a href="https://telegram.org" class="text-gray-600 hover:text-gray-900"><i class="fab fa-telegram"></i></a>
            </div>
        </div>
    }
}

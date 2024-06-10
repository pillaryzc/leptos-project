use leptos::{component, view, IntoView};

use crate::components::article::Article;

#[component]
pub fn Blog() -> impl IntoView {
    let articles = vec![
    ArticleData {
        title: "Boost your conversion rate",
        date: "Mar 16, 2020",
        category: "Marketing",
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.",
        author_name: "Michael Foster",
        author_title: "Co-Founder / CTO",
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
    },
    // Add more ArticleData instances as needed
];

    view! {
        <div class="bg-white py-24 sm:py-32">
            <div class="mx-auto max-w-7xl px-6 lg:px-8">
                <div class="mx-auto max-w-2xl lg:mx-0">
                    <h2 class="text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">From the blog</h2>
                    <p class="mt-2 text-lg leading-8 text-gray-600">Learn how to grow your business with our expert advice.</p>
                </div>
                <div class="mx-auto mt-10 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 border-t border-gray-200 pt-10 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-3">
                    {articles.iter().map(|article| {
                        view! {
                            <Article data={article} />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
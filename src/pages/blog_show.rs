use leptos::{component, view, IntoView};

use crate::components::article::{Article, ArticleData};

#[component]
pub fn Blog() -> impl IntoView {
    let articles = vec![
    ArticleData {
        title: "Boost your conversion rate".to_string(),
        date: "Mar 16, 2020".to_string(),
        category: "Marketing".to_string(),
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.".to_string(),
        author_name: "Michael Foster".to_string(),
        author_title: "Co-Founder / CTO".to_string(),
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    },
    // Add more ArticleData instances as needed
    ArticleData {
        title: "Boost your conversion rate".to_string(),
        date: "Mar 16, 2020".to_string(),
        category: "Marketing".to_string(),
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.".to_string(),
        author_name: "Michael Foster".to_string(),
        author_title: "Co-Founder / CTO".to_string(),
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    },
    ArticleData {
        title: "Boost your conversion rate".to_string(),
        date: "Mar 16, 2020".to_string(),
        category: "Marketing".to_string(),
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.".to_string(),
        author_name: "Michael Foster".to_string(),
        author_title: "Co-Founder / CTO".to_string(),
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    },
    ArticleData {
        title: "Boost your conversion rate".to_string(),
        date: "Mar 16, 2020".to_string(),
        category: "Marketing".to_string(),
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.".to_string(),
        author_name: "Michael Foster".to_string(),
        author_title: "Co-Founder / CTO".to_string(),
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    },
    ArticleData {
        title: "Boost your conversion rate".to_string(),
        date: "Mar 16, 2020".to_string(),
        category: "Marketing".to_string(),
        content: "Illo sint voluptas. Error voluptates culpa eligendi. Hic vel totam vitae illo. Non aliquid explicabo necessitatibus unde. Sed exercitationem placeat consectetur nulla deserunt vel. Iusto corrupti dicta.".to_string(),
        author_name: "Michael Foster".to_string(),
        author_title: "Co-Founder / CTO".to_string(),
        author_image: "https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
    },
];

    view! {
        <div class="bg-gradient-to-r from-indigo-900 via-purple-900 to-indigo-900 py-24 sm:py-32 transition-all duration-500">
            <div class="mx-auto max-w-7xl px-6 lg:px-8">
                <div class="mx-auto max-w-2xl lg:mx-0 text-center bg-white bg-opacity-10 p-8 rounded-lg shadow-lg backdrop-filter backdrop-blur-lg">
                    <h2 class="text-4xl font-extrabold tracking-tight text-white sm:text-5xl">
                        From the blog
                    </h2>
                    <div class="mt-2 mb-4 mx-auto h-1 w-24 bg-gradient-to-r from-purple-400 to-indigo-400 rounded-full"></div>
                    <p class="mt-4 text-lg leading-8 text-gray-300">
                        Learn how to grow your business with our expert advice.
                    </p>
                </div>
                <div class="mx-auto mt-12 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 border-t border-gray-600 pt-12 sm:mt-16 sm:pt-16 lg:mx-0 lg:max-w-none lg:grid-cols-3">
                    {articles.into_iter().map(|article| {
                        view! {
                            <div class="bg-white bg-opacity-90 rounded-lg shadow-lg p-6 transform transition-transform hover:-translate-y-2 backdrop-filter backdrop-blur-lg">
                                <Article data={article} />
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

use leptos::{component, view, IntoView};

// 定义结构体，带生命周期 'a
pub struct ArticleData<'a> {
    title: &'a str,
    date: &'a str,
    category: &'a str,
    content: &'a str,
    author_name: &'a str,
    author_title: &'a str,
    author_image: &'a str,
}

#[component]
pub fn Article<'articleData>(data: &'articleData ArticleData<'articleData>) -> impl IntoView {
    let date = data.date.clone();
    let category = data.category.clone();
    let title = data.title.clone();
    let content = data.content.clone();
    let author_image = data.author_image.clone();
    let author_name = data.author_name.clone();
    let author_title = data.author_title.clone();

    view! {
        <article class="flex max-w-xl flex-col items-start justify-between">
            <div class="flex items-center gap-x-4 text-xs">
                 <time datetime={date.clone()} class="text-gray-500">{date}</time>
                <a href="#" class="relative z-10 rounded-full bg-gray-50 px-3 py-1.5 font-medium text-gray-600 hover:bg-gray-100">{category}</a>
            </div>
            <div class="group relative">
                <h3 class="mt-3 text-lg font-semibold leading-6 text-gray-900 group-hover:text-gray-600">
                    <a href="#">
                        <span class="absolute inset-0"></span>
                        {title}
                    </a>
                </h3>
                <p class="mt-5 line-clamp-3 text-sm leading-6 text-gray-600">{content}</p>
            </div>
            <div class="relative mt-8 flex items-center gap-x-4">
                <img src={author_image} alt="" class="h-10 w-10 rounded-full bg-gray-50"></img>
                <div class="text-sm leading-6">
                    <p class="font-semibold text-gray-900">
                        <a href="#">
                            <span class="absolute inset-0"></span>
                            {author_name}
                        </a>
                    </p>
                    <p class="text-gray-600">{author_title}</p>
                </div>
            </div>
        </article>
    }
}
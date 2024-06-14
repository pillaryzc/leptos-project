use leptos::{component, view, IntoView};

// 定义结构体，带生命周期 'a
pub struct ArticleData{
    pub title:  String,
    pub date: String,
    pub category: String,
    pub content: String,
    pub author_name:String,
    pub author_title: String,
    pub author_image:String,
}

#[component]
pub fn Article(data:  ArticleData) -> impl IntoView {

    let category = data.category;
    let title = data.title;
    let content = data.content;
    let author_image = data.author_image;
    let author_name = data.author_name;
    let author_title = data.author_title;

    view! {
        <article class="flex max-w-xl flex-col items-start justify-between">
            <div class="flex items-center gap-x-4 text-xs">
                 <time datetime={author_title.clone()} class="text-gray-500">{author_title.clone()}</time>
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
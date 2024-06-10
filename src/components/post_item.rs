use leptos::*;

#[derive(Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub summary: String,
}

#[component]
pub fn PostItem(post: Post) -> impl IntoView {
    view! {
        <div class="post-item">
            <h2>{&post.title}</h2>
            <p>{&post.summary}</p>
            <a href={format!("/post/{}", post.id)}>Read more</a>
        </div>
    }
}

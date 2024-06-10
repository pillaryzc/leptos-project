use leptos::*;
use crate::components::post_item::PostItem;

#[derive(Clone)]
struct Post {
    id: u32,
    title: String,
    summary: String,
}

#[component]
pub fn PostList() -> impl IntoView {
    let posts = vec![
        Post {
            id: 1,
            title: "First Post".into(),
            summary: "This is the summary of the first post.".into(),
        },
        Post {
            id: 2,
            title: "Second Post".into(),
            summary: "This is the summary of the second post.".into(),
        },
    ];

    view! {
        <div>
            // {posts.iter().map(|post| view! { cx, <PostItem post={post.clone()} /> }).collect_view(cx)}
        </div>
    }
}

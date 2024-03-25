use leptos::*;
use crate::utils::{fetch_post, BlogPost};
use leptos_router::*;

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let post_id = params().get("id").cloned().unwrap_or_default();
    let post: BlogPost = fetch_post(post_id);

    view! {
        <article class="prose lg:prose-xl p-4">
            <h1>{post.metadata.title}</h1>
            <div class="metadata">
                <span>{"Published: "}{post.metadata.publish_date}</span>
                <span>{", Last Edited: "}{post.metadata.edit_date}</span>
            </div>
            <p>{post.metadata.preview}</p>
            <div inner_html={post.text} />
        </article>
    }
}


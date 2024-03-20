use leptos::*;
use leptos_meta::*;
use crate::components::{Header, Page};
use crate::utils::fetch_all_posts;

#[component]
pub fn Home() -> impl IntoView {
    let posts = fetch_all_posts();

    view! {
        <Title text="Home"/>
        <Page>
            <Header/>
            <section class="text-gray-600 body-font">
                <div class="container px-5 py-24 mx-auto">
                    <div class="flex flex-col text-center w-full mb-20">
                        <h1 class="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">"Simon's Blog"</h1>
                        <p class="lg:w-2/3 mx-auto leading-relaxed text-base">"Welcome to my blog. Here are the latest posts:"</p>
                    </div>
                    <div class="flex flex-wrap -m-4">
                        <For each=move || posts.values().cloned().collect::<Vec<_>>() key=|post| post.metadata.title.clone() children=move |post| {
                            let post_id = post.metadata.title.to_lowercase().replace(" ", "-"); // Simplistic slug generation
                            view! {
                                <div class="p-4 md:w-1/3">
                                    <div class="h-full border-2 border-gray-200 border-opacity-60 rounded-lg overflow-hidden">
                                        <h2 class="text-lg text-gray-900 font-medium title-font mb-2">
                                            <a href={format!("/posts/{}", post_id)}>{post.metadata.title.clone()}</a>
                                        </h2>
                                        <p class="leading-relaxed">{post.metadata.preview.clone()}</p>
                                    </div>
                                </div>
                            }
                        }/>
                    </div>
                </div>
            </section>
        </Page>
    }
}


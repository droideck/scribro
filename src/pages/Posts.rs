use leptos::*;
use leptos_meta::*;
use crate::utils::fetch_all_posts;

#[component]
pub fn Posts() -> impl IntoView {
    let posts = fetch_all_posts();

    view! {
        <section class="text-gray-600 body-font">
            <div class="container px-5 py-24 mx-auto">
                <div class="flex flex-col text-center w-full mb-20">
                    <h1 class="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">"Simon's Blog"</h1>
                    <p class="lg:w-2/3 mx-auto leading-relaxed text-base">"Welcome to my blog. Here are the latest posts:"</p>

                </div>
                <div class="flex flex-wrap -m-4">
                    <For
                        each= move || {posts.clone().into_iter()}
                        key=|(key, _post)| key.clone()
                        children=move |post| {
                            view! {
                                <div class="p-4 md:w-1/3">
                                    <div class="h-full border-2 border-gray-200 border-opacity-60 rounded-lg overflow-hidden">
                                        <div class="p-6">
                                            <h2 class="tracking-widest text-xs title-font font-medium text-gray-400 mb-1">"Simon Pichugin"</h2>
                                            <h1 class="title-font text-lg font-medium text-gray-900 mb-3">{format!("{}", post.1.metadata.title)}</h1>
                                            <p class="leading-relaxed mb-3">{format!("{}", post.1.metadata.preview)}</p>
                                            <a href={format!("/posts/{}", post.0)} class="text-indigo-500 inline-flex items-center">"Read More"
                                                <svg class="w-4 h-4 ml-2" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="M5 12h14"></path>
                                                    <path d="M12 5l7 7-7 7"></path>
                                                </svg>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </section>
    }
}

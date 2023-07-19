use leptos::*;
use leptos_meta::*;
use crate::components::Header::*;
use crate::components::Page::*;
use crate::components::PostContent::*;
use crate::utils::fetch_post;

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    let (preview, set_preview) = create_signal(cx, false);
    let post = fetch_post("hello-world".to_string());

    view! { cx,
        <Title text="Home"/>
        <Page>
            <Header/>
            <section class="text-gray-600 body-font flex flex-col items-center justify-center text-center">
                <h1 class="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">"Simon's Blog"</h1>
                <div class="flex justify-center items-center w-full">
                    <button class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" on:click=move |_| set_preview.update(|n| *n = !*n)>
                        "Preview"
                    </button>
                </div>
                <div class="container px-5 py-24 mx-auto flex flex-col items-center justify-center">
                    <div class="flex flex-col text-center w-full mb-20">
                        {move || if preview() {
                            view! { cx, <PostContent text=post.clone().text /> }.into_view(cx)
                        } else {
                            view! { cx, <div>{post.clone().metadata.preview}</div> }.into_view(cx)
                        }}
                    </div>
                </div>
            </section>
        </Page>
    }
}

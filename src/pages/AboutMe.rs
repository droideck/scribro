use leptos::*;
use leptos_meta::*;
use crate::components::Header::*;
use crate::components::Page::*;

#[component]
pub fn AboutMe() -> impl IntoView {
    let description = String::from("Hello and welcome to my blog.
    I plan to share my expirience and thoughts on different subjects and hobby projects.");

    view! {
        <Title text="Home"/>
        <Page>
            <Header/>
            <section class="text-gray-600 body-font">
                <div class="container px-5 py-24 mx-auto">
                    <div class="flex flex-col text-center w-full mb-20">
                        <h1 class="sm:text-3xl text-2xl font-medium title-font mb-4 text-gray-900">"Simon's Blog"</h1>
                        <p class="lg:w-2/3 mx-auto leading-relaxed text-base">{description}</p>
                    </div>
                </div>
            </section>
        </Page>
    }
}

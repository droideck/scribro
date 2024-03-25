use leptos::*;
use leptos_meta::*;
use crate::utils::fetch_all_posts;
use crate::components::Header::*;
use crate::pages::Posts::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <main>
            <Header/>
            <Posts/>
        </main>
    }
}

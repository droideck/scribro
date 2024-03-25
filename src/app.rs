use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::Home::*;
use crate::pages::Post::*;
use crate::pages::Posts::*;
use crate::pages::AboutMe::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App() -> impl IntoView {
    let formatter = |text| format!("{text} - Simon Pichugin");
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/scribro.css"/>
        <Title formatter/>
        <Meta
            name="description"
            content="Simon's personal blog"
        />
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Routes>
                <Route ssr=SsrMode::Async path="" view=Home/>
                <Route ssr=SsrMode::Async path="/posts" view=Posts/>
                <Route ssr=SsrMode::Async path="/about-me" view=AboutMe/>
                <Route ssr=SsrMode::Async path="/posts/:id" view=Post/>
            </Routes>
        </Router>
    }
}


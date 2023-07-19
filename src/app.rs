use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::Home::*;
use crate::pages::Posts::*;
use crate::pages::AboutMe::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let formatter = |text| format!("{text} - Simon Pichugin");
    provide_meta_context(cx);

    view! { cx,
        <Html lang="en"/>
        <Stylesheet id="leptos" href="/pkg/scribro.css"/>
        <Title formatter/>
        <Meta
            name="description"
            content="Simon's personal blog"
        />
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <Routes>
                <Route ssr=SsrMode::Async path="" view=  move |cx| view! { cx, <Home/> }/>
                <Route ssr=SsrMode::Async path="/posts" view=  move |cx| view! { cx, <Posts/> }/>
                <Route ssr=SsrMode::Async path="/about-me" view=  move |cx| view! { cx, <AboutMe/> }/>
                // <Route path="/posts/:id" view=  move |cx| view! { cx, <Post/> }/>
            </Routes>
        </Router>
    }
}


use crate::components::Footer::*;
use leptos::*;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <div>{children(cx)} <Footer/></div> }
}

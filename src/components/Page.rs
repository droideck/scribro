use crate::components::Footer::*;
use leptos::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! { <div>{children()} <Footer/></div> }
}

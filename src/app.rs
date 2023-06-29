use yew::{function_component, html, Html};

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Simon's Blog!" }</h1>
            <span class="subtitle">{ "With Yew help with " }<i class="heart" /></span>
        </main>
    }
}
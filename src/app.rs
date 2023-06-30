use yew::{function_component, html, Html};
use scribro::components::post::Post;

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

#[function_component]
pub fn PostText() -> Html {
    html! {<Post title={"First Post"} body={"foobar"} author={"Simon"} date={"2023-06-30"}/>}
}

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Simon's Blog!" }</h1>
            <PostText />
        </main>
    }
}
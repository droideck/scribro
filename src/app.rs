use yew::{function_component, html, Html};
use scribro::components::post::Post;

#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

#[function_component]
pub fn PostText() -> Html {
    let connection = crate::db::establish_connection();
    let posts = crate::db::load_posts(&connection);

    html! {
        { for posts.into_iter().map(|post| html! {<Post title={post.title} body={post.body} author={post.author} date={post.date.to_string()}/>}) }
    }
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
use yew::prelude::*;
use crate::components::{post::Post, comments::Comments, comment::Comment, tags::Tags, admin_console::AdminConsole};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
                <h1>{ "Hello Simon's Blog!" }</h1>
                <span class="subtitle">{ "With Yew help with " }<i class="heart" /></span>
            </main>
        }
    }
}

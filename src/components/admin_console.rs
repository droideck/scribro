use yew::prelude::*;

pub struct AdminConsole;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub logged_in: bool,
}

pub enum Msg {}

impl Component for AdminConsole {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{"Welcome to the Admin Console"}</h2>
            </div>
        }
    }
}
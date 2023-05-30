use yew::prelude::*;

pub struct AdminConsole {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub loggedIn: bool,
}

pub enum Msg {}

impl Component for AdminConsole {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = Props {
            loggedIn: false,
        };
        Self { props }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{"Welcome to the Admin Console"}</h2>
            </div>
        }
    }
}

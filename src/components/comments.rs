use yew::prelude::*;

pub struct Comments;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub comments: Vec<String>,
}

impl Component for Comments {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Comments:"}</p>
            </div>
        }
    }
}


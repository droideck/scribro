use yew::prelude::*;

pub struct Comment;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub comment: String,
}

impl Component for Comment {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"My comment"}</p>
            </div>
        }
    }
}

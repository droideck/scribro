use yew::prelude::*;

pub struct Tags;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tags: Vec<String>,
}

impl Component for Tags {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Tags:"}</p>
            </div>
        }
    }
}

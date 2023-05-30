use yew::prelude::*;

pub struct Comment {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub comment: String,
}

impl Component for Comment {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = Props {
            comment: "Test".to_string(),
        };
        Self { props }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"My comment"}</p>
            </div>
        }
    }
}

use yew::prelude::*;

pub struct Tags {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tags: Vec<String>,
}

impl Component for Tags {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = Props {
            tags: vec!["Test".to_string()],
        };
        Self { props }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Tags:"}</p>
            </div>
        }
    }
}

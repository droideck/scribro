use yew::prelude::*;

pub struct Post {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub body: String,
    pub author: String,
    pub date: String,
}

impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let props = Props {
            title: "Test".to_string(),
            body: "Test".to_string(),
            author: "Test".to_string(),
            date: "Test".to_string(),
        };
        Self { props }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{"Welcome to the Blog"}</h2>
                <p>{"Test Test Test"}</p>
            </div>
        }
    }
}
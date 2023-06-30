use yew::prelude::*;
use chrono::prelude::*;


pub struct Post {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub body: String,
    pub author: String,
    pub date: Option<String>,
}

impl Component for Post {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut props = ctx.props().clone();
        let _now = Local::now();
        props.date = Some(_now.format("%Y-%m-%d").to_string());

        Self {
            props,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ format!("Title: {}", &self.props.title) }</p>
                <p>{ format!("Body: {}", &self.props.body) }</p>
                <p>{ format!("Author: {}", &self.props.author) }</p>
                <p>{ format!("Date: {}", &self.props.date.as_ref().unwrap_or(&"N/A".to_string())) }</p>
            </div>
        }
    }
}
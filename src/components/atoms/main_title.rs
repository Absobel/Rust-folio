use yew::prelude::*;
use::stylist::{
    style, 
    yew::styled_component,
    Style,
};




#[derive(PartialEq, Properties)]
pub struct Properties {
    pub title: String,
}




fn stylesheet() -> Style {
    style! {
        r#"
            color: red;
        "#
    }.unwrap()
}



#[styled_component(MainTitle)]
pub fn main_title(properties: &Properties) -> Html {
    html! {
        <h1 class={stylesheet()}>
            {&properties.title}
        </h1>
    }
}
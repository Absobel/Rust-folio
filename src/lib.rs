use yew::prelude::*;
use::stylist::{
    style, 
    yew::styled_component,
    Style,
};
use gloo::{console::log};
use serde::{Serialize, Deserialize};

mod components;

use components::atoms::main_title::MainTitle;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div class={css!("color:aqua;")}>
            <MainTitle title="THIS IS MINE"/>
            <p>{"This is a paragraph"}</p>
        </div>
    }
}
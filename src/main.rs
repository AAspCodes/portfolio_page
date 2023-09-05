use stylist::yew::styled_component;
use yew::prelude::*;

mod contentbox;
use contentbox::*;

mod headpanel;
use headpanel::*;

mod sidepanel;
use sidepanel::*;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = stylist::Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={stylesheet}>
            <div class="app">
                <Headpanel/>
                <div class="Content">
                    <Sidepanel/>
                    <Contentbox/>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

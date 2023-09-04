use stylist::yew::styled_component;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = stylist::Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={stylesheet}>
            <div class="app">
                <Headpanel/>
                <Sidepanel/>
            </div>
        </div>
    }
}

#[styled_component(Headpanel)]
fn head_panel()->Html {
    html!(
        <div class="Headpanel">
            <h1> {"Portfolio Page"} </h1>
        </div>
    )

}

#[styled_component(Sidepanel)]
fn side_panel()->Html {
    html!(
        <div class="sidepanel">
            <Navlist/>
        </div>
    )

}

#[styled_component(Navlist)]
fn nav_list() -> Html {
    let items = {1..5}.collect::<Vec<_>>();
    html!(
            <ul class="nav-list">
                {items.into_iter().map(|i| {
                    html!{<li class="panelitem">{format!("item {}", i)}</li>}
                }).collect::<Html>() }
            </ul>

    )
}


fn main() {
    yew::Renderer::<App>::new().render();
}

use gloo_console::log;
use stylist::yew::styled_component;
use yew::{html, Callback, Html};

// #[derive(PartialEq, Properties)]
// pub struct SidepanelProps {
//     pub value: &'static mut UseStateHandle<&'static str>,
// }

#[styled_component(Sidepanel)]
// pub fn side_panel(props: &SidepanelProps ) -> Html {
pub fn side_panel() -> Html {
    // let SidepanelProps {value} = props;
    html!(
        <div class="Sidepanel">
            <Navlist/>
        </div>
    )
}

// #[derive(PartialEq, Properties)]
// pub struct NavlistProps {
//     pub value: &'static mut UseStateHandle<&'static str>,
// }

#[styled_component(Navlist)]
// pub fn nav_list(props: &NavlistProps) -> Html {
pub fn nav_list() -> Html {
    // let NavlistProps {value} = props;
    let items = { 1..5 }.collect::<Vec<_>>();
    html!(
            <div class="nav-list">
                {items.into_iter().map(move |i| {
                    html!{<button onclick={Callback::from(move |_| click_button(i))}>{format!("item {}", i)}</button>}
                }).collect::<Html>() }
            </div>

    )
}

fn click_button(i: i32) {
    log!(format!("clicked {}", i))
}

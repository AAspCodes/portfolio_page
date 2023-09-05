use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component(Headpanel)]
pub fn head_panel() -> Html {
    html!(
        <div class="Headpanel">
            <h1> {"Portfolio Page"} </h1>
        </div>
    )
}

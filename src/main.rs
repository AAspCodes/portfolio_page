use stylist::{
    css,
    yew::{styled_component, Global},
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Inner/>
    }
}

#[styled_component]
pub fn Inner() -> Html {
    html! {
        <>
        <Global css={css!(
            r#"
                html, body {
                    font-family: sans-serif;

                    padding: 0;
                    margin: 0;

                    display: flex;
                    justify-content: top;
                    align-items: center;
                    min-height: 100vh;
                    flex-direction: column;

                    background-color: rgb(50,50,50);
                    color: white;
                }

                .HeaderPanel {
                    min-height: 20vh;
                    min-width: 50vh;
                    background-color: rgb(255,0,0);
                }
            "#,
    )} />
        <HeaderPanel/>
        <h1>{ "Hello World" }</h1>
        </>
    }
}

#[styled_component]
fn HeaderPanel() -> Html {
    html! {
        <div class="HeaderPanel">
        <div> {"header"} </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

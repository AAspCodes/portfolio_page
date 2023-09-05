use yew::{function_component, html, Html};

// #[derive(PartialEq, Properties)]
// pub struct ContentboxProps {
//     pub value: UseStateHandle<&'static str>,
// }

#[function_component]
// pub fn Contentbox(props: &ContentboxProps) -> Html {
pub fn Contentbox() -> Html {
    // let ContentboxProps {value} = props;
    html! {
        <div class="Contentbox">
            <h1>{"hello"}</h1>
        </div>
    }
}

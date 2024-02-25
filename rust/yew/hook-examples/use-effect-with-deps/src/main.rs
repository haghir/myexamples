use yew::prelude::*;
use log::info;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    let is_loading = props.is_loading.clone();

    use_effect_with_deps(
        move |val| {
            info!(" Is loading prop changed! {}", val);
        },
        is_loading,
    );

    html! { <>{"Am I loading? - "}{is_loading}</> }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HelloWorld is_loading={true}/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

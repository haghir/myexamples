use yew::prelude::*;
use log::info;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub step: usize,
}

#[function_component(UseMemo)]
fn memo(props: &Props) -> Html {
    // Will only get recalculated if `props.step` value changes
    let message = use_memo(
        |step| {
            info!("A closure passed to use_memo was called with {}.", step);
            format!("{}. Do Some Expensive Calculation", step)
        },
        props.step,
    );

    html! {
        <div>
            <span>{ (*message).clone() }</span>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let step: UseStateHandle<usize> = use_state(|| 0);

    let on_click = {
        let step = step.clone();
        Callback::from(move |_| {
            step.set(*step + 1);
        })
    };

    html! {
        <>
            <UseMemo step={ *step / 2 }/>
            <button onclick={ on_click }>{ "Increment" }</button>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

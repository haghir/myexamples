use yew::prelude::*;
use log::info;

#[function_component(UseState)]
fn state() -> Html {
    info!("state() was called.");

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter))
    };

    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component(UseStateEq)]
fn state_eq() -> Html {
    info!("state_eq() was called.");

    let counter = use_state_eq(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter))
    };

    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <UseState/>
            <UseStateEq/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

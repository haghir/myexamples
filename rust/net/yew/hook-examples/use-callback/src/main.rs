use yew::prelude::*;
use log::info;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<String, String>,
}

#[function_component(MyComponent)]
fn my_component(props: &Props) -> Html {
    info!("my_component() was called.");
    let greeting = props.callback.emit("Yew".to_string());

    html! {
        <>{ &greeting }</>
    }
}

#[function_component(UseCallback)]
fn callback() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            info!("onclick");
            counter.set(*counter + 1)
        })
    };

    // It can also be used for events, this callback depends on `counter`.
    let oncallback = {
        let counter = counter.clone();
        use_callback(move |_e, counter| {
            info!("oncallback: {}", **counter);
            let _ = **counter;
        }, counter)
    };

    // This callback depends on (), so it's created only once, then MyComponent
    // will be rendered only once even when you click the button mutiple times.
    let callback = {
        let counter = counter.clone();
        use_callback(move |name, _| {
            info!("callback");
            format!("Current value?: {}", *counter)
        }, ())
    };

    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <button onclick={oncallback}>{ "Callback" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
            <MyComponent {callback} />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<UseCallback>::new().render();
}

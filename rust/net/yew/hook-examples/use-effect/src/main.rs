use yew::prelude::*;
use log::info;

#[function_component(UseEffect)]
fn effect() -> Html {
    info!("effect() was called.");

    let counter = use_state(|| 0);

    let counter_one = counter.clone();
    use_effect(move || {
        info!("A closure passed to use_effect was called.");

        // Make a call to DOM API after component is rendered
        if *counter_one % 2 == 0 {
            gloo::utils::document().set_title(&format!("You clicked {} times", *counter_one));
        }

        // Perform the cleanup
        || {
            info!("cleanup closure was called.");
            gloo::utils::document().set_title(&format!("You clicked 0 times"))
        }
    });

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            info!("onclick was called.");
            counter.set(*counter + 1)
        })
    };

    let h = html! {
        <button {onclick}>{ format!("Increment to {}", *counter) }</button>
    };

    info!("effect() was finished");

    h
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<UseEffect>::new().render();
}

use yew::prelude::*;
use log::info;

#[function_component]
fn ManuallyUpdatedDate() -> Html {
    info!("ManuallyUpdatedDate() was called.");

    let trigger = use_force_update();
    let onclick = use_state(move || {
        info!("use_state that defines a onclick callback.");
        Callback::from(move |_| {
            info!("onclick was called");
            trigger.force_update()
        })
    });
    let last_update = js_sys::Date::new_0().to_utc_string();
    html! {
        <div>
            <button onclick={&*onclick}>{"Update now!"}</button>
            <p>{"Last updated: "}{last_update}</p>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ManuallyUpdatedDate>::new().render();
}

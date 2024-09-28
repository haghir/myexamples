use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement};
use yew::prelude::*;
use log::info;

#[function_component(UseNodeRef)]
pub fn node_ref_hook() -> Html {
    info!("node_ref_hook() was called.");

    let div_ref = use_node_ref();

    {
        let div_ref = div_ref.clone();

        use_effect_with_deps(
            // The following closure will be called after the <div> was created,
            // and passed a reference to it.
            |div_ref| {
                info!("A closure passed to use_effect_with_deps was called.");

                let div = div_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                info!("{:?}", div);

                let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
                    web_sys::console::log_1(&"Clicked!".into());
                }));

                div.add_event_listener_with_callback(
                    "click",
                    listener.as_ref().unchecked_ref(),
                )
                    .unwrap();

                move || {
                    div.remove_event_listener_with_callback(
                        "click",
                        listener.as_ref().unchecked_ref(),
                    )
                        .unwrap();
                }
            },
            div_ref,
        );
    }

    let h = html! {
        <div ref={div_ref}>
            { "Click me and watch the console log!" }
        </div>
    };

    info!("{:?}", h);

    h
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<UseNodeRef>::new().render();
}

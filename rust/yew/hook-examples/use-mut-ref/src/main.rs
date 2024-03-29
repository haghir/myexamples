use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(UseRef)]
fn ref_hook() -> Html {
    let message = use_state(|| "".to_string());
    let message_count = use_mut_ref(|| 0);

    let onclick = Callback::from(move |e| {
        let window = gloo::utils::window();

        if *message_count.borrow_mut() > 3 {
            window.alert_with_message("Message limit reached");
        } else {
            *message_count.borrow_mut() += 1;
            window.alert_with_message("Message sent");
        }
    });

    let onchange = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value())
        })
    };

    html! {
        <div>
            <input {onchange} value={(*message).clone()} />
            <button {onclick}>{ "Send" }</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<UseRef>::new().render();
}

//! Note that, according to the following docment, it shuould be avoided to
//! use RefCell for the Properties because Yew get not to know when the state
//! has been changed. But it seems not to be prohibited.
//! https://yew.rs/docs/next/concepts/function-components/properties

use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;

#[derive(Properties, PartialEq, Debug)]
struct ContextBody {
    counter: u32,
}

impl Clone for ContextBody {
    fn clone(&self) -> Self {
        // This function is never called.
        info!("ContextBody.clone() was called.");
        Self {
            counter: self.counter,
        }
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
struct Context {
    body: Rc<RefCell<ContextBody>>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
struct ChildProps {
    on_increment: Callback<()>,
}

#[function_component(Child)]
fn child(ChildProps { on_increment }: &ChildProps) -> Html {
    let ctx = use_context::<Context>().unwrap();

    let on_increment = {
        let on_increment = on_increment.clone();
        Callback::from(move |_| {
            on_increment.emit(());
        })
    };

    html! {
        <div>
            <p>{ ctx.body.borrow().counter }</p>
            <button onclick={ on_increment }>{ "Increment" }</button>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let ctx = use_state(move || Context {
        body: Rc::new(RefCell::new(ContextBody {
            counter: 0,
        }))
    });

    let on_increment = {
        let ctx = ctx.clone();
        Callback::from(move |_| {
            info!("before updated: {:p}", &(*ctx).body.borrow().counter);
            {
                let mut body = (*ctx.body).borrow_mut();
                body.counter += 1;
                if body.counter %2 == 0 {
                    ctx.set((*ctx).clone());
                } else {
                    // body.counter was updated, but not re-rendered.
                }
            }
            info!("after updated: {:p}", &(*ctx).body.borrow().counter);
        })
    };

    html! {
        <ContextProvider<Context> context={(*ctx).clone()}>
            <p>{ ctx.body.borrow().counter }</p>
            <Child on_increment={ on_increment }/>
        </ContextProvider<Context>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use std::rc::Rc;
use log::info;

/// reducer's Action
enum CounterAction {
    Double,
    Square,
}

/// reducer's State
#[derive(PartialEq)]
struct CounterState {
    counter: i32,
}

impl Default for CounterState {
    fn default() -> Self {
        Self { counter: 1 }
    }
}

impl Reducible for CounterState {
    /// Reducer Action Type
    type Action = CounterAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            CounterAction::Double => self.counter * 2,
            CounterAction::Square => self.counter.pow(2),
        };

        Self { counter: next_ctr }.into()
    }
}

#[function_component(UseReducer)]
fn reducer() -> Html {
    info!("reducer() was called.");

    // The use_reducer hook takes an initialization function which will be called only once.
    let counter = use_reducer(CounterState::default);

    let double_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(CounterAction::Double))
    };
    let square_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(CounterAction::Square))
    };

    html! {
        <>
            <div id="result">{ counter.counter }</div>

            <button onclick={double_onclick}>{ "Double" }</button>
            <button onclick={square_onclick}>{ "Square" }</button>
        </>
    }
}

#[function_component(UseReducerEq)]
fn reducer_eq() -> Html {
    info!("reducer_eq() was called.");

    // The use_reducer hook takes an initialization function which will be called only once.
    let counter = use_reducer_eq(CounterState::default);

    let double_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(CounterAction::Double))
    };
    let square_onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.dispatch(CounterAction::Square))
    };

    html! {
        <>
            <div id="result">{ counter.counter }</div>

            <button onclick={double_onclick}>{ "Double" }</button>
            <button onclick={square_onclick}>{ "Square" }</button>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <UseReducer/>
            <UseReducerEq/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

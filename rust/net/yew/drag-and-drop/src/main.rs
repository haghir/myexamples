//! # Events fired on the drag target.
//!
//! * drag: The drag event is fired every few hundred milliseconds as an
//!     element or text selection is being dragged by the user.
//!
//! * dragstart: The dragstart event is fired when the user starts dragging
//!     an element or text selection.
//!
//! * dragend: The dragend event is fired when a drag operation is being ended
//!     (by releasing a mouse button or hitting the escape key).
//!
//! # Events fired on the drop target.
//!
//! * dragover: The dragover event is fired when an element or text selection is
//!     being dragged over a valid drop target (every few hundred milliseconds).
//!
//! * dragenter: The dragenter event is fired when a dragged element or text selection
//!     enters a valid drop target.
//!
//! * dragleave: The dragleave event is fired when a dragged element or text selection
//!     leaves a valid drop target.
//!
//! * drop: The drop event is fired when an element or text selection is dropped on
//!     a valid drop target.

use yew::prelude::*;
use log::info;

#[function_component(DragTarget)]
fn drag_target() -> Html {
    info!("drag_target() was called.");

    let style = use_state(|| classes!("put"));

    let on_drag = Callback::from(|_| {
        //info!("on drag");
    });

    let on_dragstart = {
        let style = style.clone();
        Callback::from(move |_| {
            info!("on dragstart");
            style.set(classes!("dragging"));
        })
    };

    let on_dragend = {
        let style = style.clone();
        Callback::from(move |_| {
            info!("on dragend");
            style.set(classes!("put"));
        })
    };

    html! {
        <div id="drag-target"
             class={ (*style).clone() }
             draggable="true"
             ondrag={ on_drag }
             ondragstart={ on_dragstart }
             ondragend={ on_dragend }></div>
    }
}

#[derive(Properties, PartialEq)]
struct DragStageProps {
    switch: bool,
    characteristic: bool,
    on_drop: Callback<bool>,
}

#[function_component(DragStage)]
fn drag_stage(DragStageProps { switch, characteristic, on_drop }: &DragStageProps) -> Html {
    info!("drag_stage() was called.");

    let on_dragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let on_dragenter = Callback::from(|_| {
        info!("on dragenter");
    });

    let on_dragleave = Callback::from(|_| {
        info!("on dragleave");
    });

    let on_drop2 = {
        let on_drop = on_drop.clone();
        let characteristic = *characteristic;

        Callback::from(move |_| {
            info!("on drop");

            let on_drop = on_drop.clone();
            on_drop.emit(characteristic);
        })
    };

    html! {
        if *switch {
            <div id="drag-from" class="drop-target">
                <DragTarget/>
            </div>
        } else {
            <div id="drag-to" class="drop-target"
                 ondragover={ on_dragover }
                 ondragenter={ on_dragenter }
                 ondragleave={ on_dragleave }
                 ondrop={ on_drop2 }></div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    info!("app() was called.");

    let switch = use_state(|| true);

    let on_drop = {
        let switch = switch.clone();
        Callback::from(move |b: bool| {
            let switch = switch.clone();
            switch.set(b);
        })
    };

    html! {
        <main>
            <DragStage switch={ *switch } characteristic={ true } on_drop={ on_drop.clone() }/>
            <DragStage switch={ !*switch } characteristic={ false } on_drop={ on_drop.clone() }/>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

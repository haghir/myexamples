use std::cell::RefCell;
use yew::prelude::*;
use std::rc::Rc;

#[allow(unused_imports)]
use log::info;

#[derive(Clone, PartialEq)]
struct ItemType {
    color: String,
}

impl ItemType {
    fn new(color: &str) -> Rc<Self> {
        Rc::new(Self {
            color: color.to_string(),
        })
    }
}

#[derive(Clone, PartialEq)]
struct Item {
    item_type: Rc<ItemType>,
    x: i32,
    y: i32,
}

impl Item {
    fn new(item_type: Rc<ItemType>, x: i32, y: i32) -> Self {
        Self {
            item_type: item_type.clone(),
            x,
            y,
        }
    }
}

#[derive(Clone, PartialEq)]
struct DragContext {
    item_type: Rc<ItemType>,
    x: i32,
    y: i32,
}

#[derive(Properties, PartialEq)]
struct ListItemProps {
    item_type: Rc<ItemType>,
    index: usize,
    on_dragstart: Callback<(usize, i32, i32)>,
}

#[derive(Properties, PartialEq)]
struct ListProps {
    item_types: Rc<Vec<Rc<ItemType>>>,
    on_dragstart: Callback<(usize, i32, i32)>,
}

#[derive(Properties, PartialEq)]
struct CanvasProps {
    items: Rc<RefCell<Vec<Item>>>,
    on_drop: Callback<(i32, i32)>,
}

#[function_component(ListItem)]
fn list_item(ListItemProps { item_type, index, on_dragstart }: &ListItemProps) -> Html {
    let index = *index;

    let on_dragstart = {
        let on_dragstart = on_dragstart.clone();
        Callback::from(move |e: DragEvent| {
            on_dragstart.emit((index, e.offset_x(), e.offset_y()));
        })
    };

    html! {
        <li style={ format!("background-color: {}", item_type.color) }
            ondragstart={ on_dragstart }
            draggable="true">
        </li>
    }
}

#[function_component(List)]
fn list(ListProps { item_types, on_dragstart }: &ListProps) -> Html {
    let list_items = item_types.iter().enumerate().map(|(index, item_type)| {
        let on_dragstart = on_dragstart.clone();
        html! {
            <ListItem item_type={ item_type.clone() } index={ index } on_dragstart={ on_dragstart }/>
        }
    });

    html! {
        <ul>
            { for list_items }
        </ul>
    }
}

#[function_component(Canvas)]
fn canvas(CanvasProps { items, on_drop }: &CanvasProps) -> Html {
    let items = items.borrow();

    let item_nodes = items.iter().map(|item| {
        let style = format!("background-color: {}; left: {}px; top: {}px", item.item_type.color, item.x, item.y);
        html! {
            <div class={ classes!("item") } style={ style }>
            </div>
        }
    });

    let on_dragover = Callback::from(|e: DragEvent| {
        e.prevent_default();
    });

    let on_drop = {
        let on_drop = on_drop.clone();
        Callback::from(move |e: DragEvent| {
            on_drop.emit((e.offset_x(), e.offset_y()));
        })
    };

    html! {
        <div id={ "canvas" } ondrop={ on_drop } ondragover={ on_dragover }>
            { for item_nodes }
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let item_types = use_memo(|_| {
        vec![
            ItemType::new("#F0F8FF"),
            ItemType::new("#006400"),
            ItemType::new("#FFB6C1"),
            ItemType::new("#CD853F"),
            ItemType::new("#FFFF00"),
            ItemType::new("#9ACD32"),
            ItemType::new("#40E0D0"),
            ItemType::new("#4682B4"),
        ]
    }, ());

    let items: UseStateHandle<Rc<RefCell<Vec<Item>>>> = use_state(|| Rc::new(RefCell::new(vec![])));

    let drag_context: UseStateHandle<Option<DragContext>> = use_state(|| None);

    let on_dragstart = {
        let item_types = item_types.clone();
        let drag_context = drag_context.clone();
        Callback::from(move |(index, x, y): (usize, i32, i32)| {
            drag_context.set(Some(DragContext {
                item_type: item_types[index].clone(),
                x,
                y,
            }));
        })
    };

    let on_drop = {
        let items = items.clone();
        let drag_context = drag_context.clone();
        Callback::from(move |(x, y): (i32, i32)| {
            if let Some(ctx) = &(*drag_context) {
                let item_type = ctx.item_type.clone();
                let mut mut_items = items.borrow_mut();
                mut_items.push(Item::new(item_type, x - ctx.x, y - ctx.y));
                items.set((*items).clone());
            }
        })
    };

    html! {
        <>
            <List item_types={ item_types.clone() } on_dragstart={ on_dragstart }/>
            <Canvas items={ (*items).clone() } on_drop={ on_drop }/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;

/// App theme
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// Main component
#[function_component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme`
        // so we deref it.
        // It derefs to `&Theme`, hence the clone
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // Every child here and their children will have access to this context.
            <Toolbar />
        </ContextProvider<Theme>>
    }
}

/// The toolbar.
/// This component has access to the context
#[function_component]
pub fn Toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// Button placed in `Toolbar`.
/// As this component is a child of `ThemeContextProvider` in the component tree,
/// it also has access to the context.
#[function_component]
pub fn ThemedButton() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
            { "Click me!" }
        </button>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

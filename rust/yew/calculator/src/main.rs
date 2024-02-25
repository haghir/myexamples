use std::fmt::{Display, Formatter};
use yew::prelude::*;
use log::info;

#[derive(PartialEq, Clone)]
enum Input {
    Clear,
    Exec,
    Add,
    Sub,
    Mul,
    Div,
    Digit(i32),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Clear => write!(f, "C"),
            Input::Exec => write!(f, "="),
            Input::Add => write!(f, "+"),
            Input::Sub => write!(f, "-"),
            Input::Mul => write!(f, "*"),
            Input::Div => write!(f, "/"),
            Input::Digit(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProps {
    input: Input,
    width: u32,
    height: u32,
    on_click: Callback<Input>,
}

#[function_component(Button)]
fn button(ButtonProps { input, width, height, on_click }: &ButtonProps) -> Html {
    info!("button() was called.");

    let label = input.to_string();

    let on_update = {
        let on_click = on_click.clone();
        let input = input.clone();

        Callback::from(move |_| {
            on_click.emit(input.clone())
        })
    };

    html! {
        <td class="button"
            rowspan={ width.to_string() }
            colspan={ height.to_string() }
            onclick={ on_update }>{ label.clone() }</td>
    }
}

#[derive(Clone)]
struct State {
    left: i32,
    right: i32,
    op: Input,
    display: String,
}

fn calc(op: &Input, left: i32, right: i32) -> Result<i32, ()> {
    match op {
        Input::Add => Ok(left + right),
        Input::Sub => Ok(left - right),
        Input::Mul => Ok(left * right),
        Input::Div => if right != 0 {
            Ok(left / right)
        } else {
            Err(())
        },
        _ => Ok(right),
    }
}

#[function_component(App)]
fn app() -> Html {
    info!("app() was called.");

    let state = use_state(|| {
        info!("use_state initialization");
        State {
            left: 0,
            right: 0,
            op: Input::Clear,
            display: String::from("0"),
        }
    });

    let on_click = {
        let state = state.clone();

        Callback::from(move |input: Input| {
            let state = state.clone();
            let mut next = (*state).clone();

            match input {
                Input::Clear => {
                    next.left = 0;
                    next.right = 0;
                    next.op = Input::Clear;
                    next.display = String::from("0");
                },
                Input::Exec => if state.op != Input::Clear {
                    next.left = 0;
                    next.right = 0;
                    next.op = Input::Clear;
                    if let Ok(n) = calc(&state.op, state.left, state.right) {
                        next.display = format!("{}", n);
                    } else {
                        next.display = String::from("Error");
                    }
                },
                Input::Digit(n) => {
                    next.right = state.right * 10 + n;
                    next.display = format!("{}", next.right);
                },
                operator => if let Ok(n) = calc(&state.op, state.left, state.right) {
                    next.left = n;
                    next.right = 0;
                    next.op = operator;
                    next.display = format!("{}", n);
                } else {
                    next.left = 0;
                    next.right = 0;
                    next.op = Input::Clear;
                    next.display = String::from("Error");
                },
            }

            state.set(next);
        })
    };

    info!("before html!");
    let h = html! {
        <table>
            <tr>
                <td colspan="4" style="text-align:right">{ state.display.clone() }</td>
            </tr>
            <tr>
                <Button input={ Input::Clear } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Div } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Mul } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Sub } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
            </tr>
            <tr>
                <Button input={ Input::Digit(7) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(8) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(9) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Add } width={ 2 } height={ 1 } on_click={ on_click.clone() } />
            </tr>
            <tr>
                <Button input={ Input::Digit(4) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(5) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(6) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
            </tr>
            <tr>
                <Button input={ Input::Digit(1) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(2) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Digit(3) } width={ 1 } height={ 1 } on_click={ on_click.clone() } />
                <Button input={ Input::Exec } width={ 2 } height={ 1 } on_click={ on_click.clone() } />
            </tr>
            <tr>
                <Button input={ Input::Digit(0) } width={ 1 } height={ 3 } on_click={ on_click.clone() } />
            </tr>
        </table>
    };
    info!("before html!");
    h
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

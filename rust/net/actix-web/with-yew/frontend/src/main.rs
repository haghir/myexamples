use common::{Request, Response};
use std::fmt::{Display, Formatter};
use yew::prelude::*;
use log::info;

#[derive(Properties, PartialEq, Clone, Debug)]
struct MessageAreaProps {
    result: Option<Response>,
}

#[function_component(MessageArea)]
fn message_area(MessageAreaProps { result }: &MessageAreaProps) -> Html {
    if let Some(res) = result {
        if let Ok(user) = &res.result {
            return html! {
                <ul>
                    <li>{ "Name: " }{ user.name.clone() }</li>
                    <li>{ "Age: " }{ user.age }</li>
                </ul>
            };
        } else if let Err(error) = &res.result {
            return html! {
                <p class="error">{ error.message.clone() }</p>
            };
        }
    }

    html! {
        <></>
    }
}

#[derive(PartialEq, Clone)]
enum Input {
    Clear,
    Submit,
    Digit(u8),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Clear => write!(f, "C"),
            Input::Submit => write!(f, "E"),
            Input::Digit(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
struct ButtonProps {
    input: Input,
    on_click: Callback<Input>,
}

#[function_component(Button)]
fn button(ButtonProps { input, on_click }: &ButtonProps) -> Html {
    let on_update = {
        let on_click = on_click.clone();
        let input = input.clone();

        Callback::from(move |_| on_click.emit(input.clone()))
    };

    html! {
        <td class="button" onclick={ on_update }>{ input.to_string() }</td>
    }
}

#[derive(Clone)]
struct State {
    value: String,
    display: String,
    result: Option<Response>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| State {
        value: String::from(""),
        display: String::from(""),
        result: None,
    });

    let on_click = {
        let state = state.clone();

        Callback::from(move |input: Input| {
            let state = state.clone();
            let mut next = (*state).clone();

            match input {
                Input::Digit(n) => {
                    next.value = format!("{}{}", next.value, n);
                    next.display += "*";
                    next.result = None;
                }
                Input::Clear => {
                    next.value = String::from("");
                    next.display = String::from("");
                    next.result = None;
                }
                Input::Submit => {
                    next.value = String::from("");
                    next.display = String::from("");
                    {
                        let state = state.clone();
                        let mut next = next.clone();
                        let request = Request {
                            pin: state.value.clone(),
                        };
                        wasm_bindgen_futures::spawn_local(async move {
                            let fetched_res: Response = gloo_net::http::Request::post("/auth")
                                .json(&request)
                                .unwrap()
                                .send()
                                .await
                                .unwrap()
                                .json()
                                .await
                                .unwrap();

                            next.result = Some(fetched_res);
                            state.set(next);
                        });
                        info!("requested.");
                    }
                }
            }

            state.set(next);
        })
    };

    html! {
        <>
            <MessageArea result={ state.result.clone() }/>
            <table>
                <tr>
                    <td class="display" colspan=3>{ "\u{00A0}" }<span>{ state.display.clone() }</span></td>
                </tr>
                <tr>
                    <Button input={ Input::Digit(7) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(8) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(9) } on_click={ on_click.clone() }/>
                </tr>
                <tr>
                    <Button input={ Input::Digit(4) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(5) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(6) } on_click={ on_click.clone() }/>
                </tr>
                <tr>
                    <Button input={ Input::Digit(1) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(2) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(3) } on_click={ on_click.clone() }/>
                </tr>
                <tr>
                    <Button input={ Input::Clear } on_click={ on_click.clone() }/>
                    <Button input={ Input::Digit(0) } on_click={ on_click.clone() }/>
                    <Button input={ Input::Submit } on_click={ on_click.clone() }/>
                </tr>
            </table>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

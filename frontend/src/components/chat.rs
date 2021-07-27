use js_sys::Date;
use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::web_sys::HtmlInputElement;

use crate::ServerMessages;

#[derive(Serialize)]
struct MessagePayload {
    #[serde(rename = "type")]
    type_: String,
    chatter: String,
    text: String,
}

impl MessagePayload {
    fn new(chatter: String, text: String) -> Self {
        Self {
            type_: "Message".to_string(),
            chatter,
            text,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub login: String,
    pub send: Callback<String>,
    pub messages: Vec<ServerMessages>,
    pub chatters: Vec<String>,
}

pub enum Actions {
    Input(String),
}

pub struct Chat {
    link: ComponentLink<Self>,
    login: String,
    message: String,
    messages: Vec<ServerMessages>,
    chatters: Vec<String>,
    send: Callback<String>,
    input_ref: NodeRef,
}

impl Component for Chat {
    type Message = Actions;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            login: props.login,
            message: "".to_string(),
            messages: props.messages,
            chatters: props.chatters,
            send: props.send,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Actions::Input(value) => {
                self.message = value;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.messages.len() != props.messages.len() {
            self.messages = props.messages;
            return true;
        }
        if self.chatters.len() != props.chatters.len() {
            self.chatters = props.chatters;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let chatter = self.login.clone();
        let message = self.message.clone();
        let login_fn = self.send.clone();
        let input_ref = self.input_ref.clone();
        let handle_send_message = Callback::from(move |_| {
            login_fn.emit(
                serde_json::to_string(&MessagePayload::new(chatter.clone(), message.clone()))
                    .unwrap(),
            );
            ConsoleService::log(
                input_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .default_value()
                    .as_str(),
            );
            input_ref.cast::<HtmlInputElement>().unwrap().set_value("");
        });

        html! {
            <main class="flex items-center justify-center w-screen h-screen">
                <div class="border-2 rounded border-gray-200 chat-container flex">
                    <div class="flex flex-col flex-1 h-full">
                        <ul class="flex flex-col flex-1 p-3">
                            {self.messages.iter().map(|it| {
                                match it {
                                    ServerMessages::Message {time, chatter, text} => {
                                        let time = Date::new(&JsValue::from_f64(time.clone()));
                                        html!{
                                            <li>
                                                {format!("{}:{} {} > {}", time.get_hours(), time.get_minutes(), chatter, &text)}
                                            </li>
                                        }
                                    }
                                    ServerMessages::JoinSystem { text } => {
                                        let time = Date::new_0();
                                        html!{
                                            <li>
                                                {format!("{}:{} >> {}", time.get_hours(), time.get_minutes(), &text)}
                                            </li>
                                        }
                                    }
                                    _ => html!{<></>}
                                }
                            }).collect::<Html>()}
                        </ul>
                        <div class="flex p-3 border-t">
                            <div class="flex items-center justify-center pr-3">
                                {format!("{} says:", self.login)}
                            </div>
                            <input
                                ref=self.input_ref.clone()
                                type="text"
                                class="border rounded border-gray-400 focus:border-blue-400 p-2 flex-1"
                                placeholder="Message"
                                oninput=self.link.callback(|e: InputData| Actions::Input(e.value))
                            />
                            <button
                                type="button"
                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ml-5"
                                onclick=handle_send_message
                            >
                                {"Send"}
                            </button>
                        </div>
                    </div>
                    <div class="flex flex-col w-1/3 h-full border-l">
                        <h1 class="text-center">{"Chatters"}</h1>
                        <ul class="flex flex-col flex-1 p-2 pt-5">
                            {self.chatters.iter().map(|it| {
                                html!{ <li>{it}</li>}
                            }).collect::<Html>()}
                        </ul>
                    </div>
                </div>
            </main>
        }
    }
}

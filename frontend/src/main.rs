use serde::{Deserialize, Serialize};
use serde_json::from_str;
use yew::format::Text;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::ConsoleService;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router::prelude::*;

mod components;
use components::chat::Chat;
use components::login::Login;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/chat"]
    ChatRoute,
    #[to = "/login"]
    LoginRoute,
    #[to = "/"]
    IndexRoute,
}

enum Actions {
    Rx(String),
    Send(String),
    Nop,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ServerMessages {
    LoginSystem {
        text: String,
    },
    JoinSystem {
        text: String,
    },
    Message {
        chatter: String,
        time: f64,
        text: String,
    },
    Chatters {
        chatters: Vec<String>,
    },
}

struct Model<STATE: RouterState = ()> {
    link: ComponentLink<Self>,
    ws: WebSocketTask,
    login: String,
    messages: Vec<ServerMessages>,
    chatters: Vec<String>,
    router: RouteAgentDispatcher<STATE>,
}

impl<STATE: RouterState> Component for Model<STATE> {
    type Message = Actions;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let in_callback = link.callback(|data: Text| Actions::Rx(data.unwrap()));
        let n_callback = link.callback(|x: WebSocketStatus| {
            ConsoleService::log("TODO: disconnect handle");
            match x {
                WebSocketStatus::Opened => ConsoleService::log(">> test"),
                _ => (),
            }
            Actions::Nop
        });

        let ws = WebSocketService::connect_text("ws://127.0.0.1:8081/ws/", in_callback, n_callback)
            .unwrap();
        Self {
            link,
            ws,
            login: "".to_string(),
            messages: vec![],
            chatters: vec![],
            router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Actions::Send(data) => {
                self.ws.send(Ok(data));
                false
            }
            Actions::Rx(data) => {
                ConsoleService::log(&data);
                match from_str::<ServerMessages>(&data) {
                    Ok(ServerMessages::LoginSystem { text }) => {
                        if text != "false" {
                            self.login = text;
                            self.router
                                .send(RouteRequest::ChangeRoute(Route::from(AppRoute::ChatRoute)));
                            return true;
                        }
                        false
                    }
                    Ok(ServerMessages::Chatters { chatters }) => {
                        self.chatters = chatters;
                        true
                    }
                    Ok(x) => {
                        self.messages.push(x);
                        true
                    }
                    Err(e) => {
                        ConsoleService::error(e.to_string().as_str());
                        false
                    }
                }
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ws_send = self.link.callback(move |data: String| Actions::Send(data));
        let login = self.login.clone();
        let messages = self.messages.clone();
        let chatters = self.chatters.clone();

        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::LoginRoute => html!{<Login send=ws_send.clone() />},
                        AppRoute::ChatRoute => html!{<Chat
                            login=login.clone()
                            messages=messages.clone()
                            chatters=chatters.clone()
                            send=ws_send.clone()
                        />},
                        AppRoute::IndexRoute => html!{<Login send=ws_send.clone() />},
                    }
                })
            />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

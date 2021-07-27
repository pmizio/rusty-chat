use serde::Serialize;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;

#[derive(Serialize)]
struct LoginPayload {
    #[serde(rename = "type")]
    type_: String,
    name: String,
}

impl LoginPayload {
    fn new(name: String) -> Self {
        Self {
            type_: "Login".to_string(),
            name,
        }
    }
}

pub enum Actions {
    Input(String),
}

pub struct Login {
    link: ComponentLink<Self>,

    login: String,
    send: Callback<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub send: Callback<String>,
}

impl Component for Login {
    type Message = Actions;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            login: "".to_string(),
            send: props.send,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Actions::Input(value) => {
                self.login = value;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let login = self.login.clone();
        let login_fn = self.send.clone();
        let handle_login = Callback::from(move |_| {
            let t = serde_json::to_string(&LoginPayload::new(login.clone())).unwrap();
            ConsoleService::log(&t);
            login_fn.emit(t);
        });

        html! {
            <main class="flex items-center justify-center w-screen h-screen">
                <div class="border-2 rounded border-gray-200 p-2 login-container flex flex-col items-center justify-center">
                    <h1 class="mb-5">{"Hi! What's your name?"}</h1>
                    <input
                        type="text"
                        class="border rounded border-gray-400 focus:border-blue-400 p-2 mb-5"
                        placeholder="Name"
                        oninput=self.link.callback(|e: InputData| Actions::Input(e.value))
                    />
                    <button
                        type="button"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        onclick=handle_login
                    >
                        {"Login"}
                    </button>
                </div>
            </main>
        }
    }
}

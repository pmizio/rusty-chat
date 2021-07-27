use actix::prelude::*;
use actix::Actor;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, Result};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use super::message::{ChatMessage, ChattersMessage, IncomingMessage, Message, SystemMessage};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Actions {
    Login { name: String },
    Message { chatter: String, text: String },
}

pub struct ChatServer {
    pub chatters: HashMap<String, Recipient<Message>>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl ChatServer {
    fn send_message(&self, addr: &Recipient<Message>, value: Result<String>) {
        match value {
            Ok(value) => {
                addr.do_send(Message(value.clone())).expect("Bad message!");
            }
            Err(_) => println!("Bad message!"),
        }
    }

    fn broadcast_message(&mut self, value: Result<String>) {
        match value {
            Ok(value) => self
                .chatters
                .retain(|_, addr| addr.do_send(Message(value.clone())).is_ok()),
            Err(_) => println!("Bad message!"),
        }
    }
}

impl Handler<IncomingMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: IncomingMessage, _ctx: &mut Context<Self>) {
        match from_str(&msg.text) {
            Ok(Actions::Login { name }) => {
                if !self.chatters.contains_key(&name) {
                    self.broadcast_message(to_string(&SystemMessage {
                        type_: "JoinSystem".to_string(),
                        text: format!("{} joined!", name),
                    }));

                    self.chatters.insert(name.clone(), msg.addr);

                    self.send_message(
                        self.chatters.get(&name).unwrap(),
                        to_string(&SystemMessage {
                            type_: "LoginSystem".to_string(),
                            text: name,
                        }),
                    );

                    self.broadcast_message(to_string(&ChattersMessage::new(
                        self.chatters.clone().into_iter().map(|(k, _)| k).collect(),
                    )));
                }
            }
            Ok(Actions::Message { chatter, text }) => {
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("This os has no time xD")
                    .as_millis();

                self.broadcast_message(to_string(&ChatMessage::new(chatter, time, text.clone())));
            }
            Err(_) => println!("Unknown action!"),
        }
    }
}

use actix::prelude::*;
use serde::Serialize;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct IncomingMessage {
    pub addr: Recipient<Message>,
    pub text: String,
}

#[derive(Serialize)]
pub struct ChatMessage {
    #[serde(rename = "type")]
    pub type_: String,
    pub chatter: String,
    pub time: u128,
    pub text: String,
}

impl ChatMessage {
    pub fn new(chatter: String, time: u128, text: String) -> Self {
        Self {
            type_: "Message".to_string(),
            chatter,
            time,
            text,
        }
    }
}

#[derive(Serialize)]
pub struct SystemMessage {
    #[serde(rename = "type")]
    pub type_: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct ChattersMessage {
    #[serde(rename = "type")]
    pub type_: String,
    pub chatters: Vec<String>,
}

impl ChattersMessage {
    pub fn new(chatters: Vec<String>) -> Self {
        Self {
            type_: "Chatters".to_string(),
            chatters,
        }
    }
}

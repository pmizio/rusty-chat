use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::collections::HashMap;

mod chat;
use chat::chat::ChatServer;
use chat::message::{IncomingMessage, Message};

struct MyWs {
    addr: Addr<ChatServer>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<Message> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                self.addr.do_send(IncomingMessage {
                    addr: ctx.address().recipient(),
                    text,
                });
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        MyWs {
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = ChatServer {
        chatters: HashMap::new(),
    }
    .start();

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .service(web::resource("/ws/").to(index))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

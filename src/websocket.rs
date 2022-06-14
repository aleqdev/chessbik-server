use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws::{self, WebsocketContext};
use chessbik_commons::WsMessage;
use std::time::Duration;


use crate::data_server::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct InternalWsMessage(pub String);

pub struct Ws {
    pub data: Addr<DataServer>,
    pub addr: String,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<InternalWsMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: InternalWsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("got message:\n{}\n", text);
                self.handle_ws_text(text, ctx);
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_secs_f32(20.), |_, ctx| {
            crate::send_to_recip(WsMessage::Hb, &ctx.address().recipient());
        });
    }
}

impl Ws {
    pub fn new(data: Addr<DataServer>, addr: impl AsRef<str>) -> Self {
        Self {
            data,
            addr: addr.as_ref().to_owned(),
        }
    }

    fn handle_ws_text(&self, text: impl AsRef<str>, ctx: &mut WebsocketContext<Self>) {
        match serde_json::from_str::<WsMessage>(text.as_ref()) {
            Err(err) => println!(
                "error: failed to deserialize ws message, ignoring:\n{}",
                err
            ),
            Ok(message) => self.handle_ws_message(message, ctx),
        }
    }

    fn handle_ws_message(&self, message: WsMessage, ctx: &mut WebsocketContext<Self>) {
        let recip = ctx.address().recipient();

        match message {
            WsMessage::CreateGame => {
                self.data.do_send(CreateGameMessage(recip));
            }
            WsMessage::RequestBoard(lobby) => {
                self.data.do_send(RequestBoardMessage(lobby, recip));
            }
            WsMessage::RequestPlayerToken => {
                self.data.do_send(RequestPlayerTokenMessage(
                    self.addr.clone(),
                    recip
                ));
            }
            WsMessage::RequestOpponentAddition(lobby, color, token, name) => {
                self.data.do_send(RequestOpponentAdditionMessage(
                    lobby,
                    color,
                    token,
                    name
                ));
            }
            WsMessage::RequestEngineAddition(lobby, color, token) => {
                self.data.do_send(RequestEngineAdditionMessage(
                    lobby,
                    color,
                    token,
                ));
            }
            WsMessage::RequestPlayerRemoval(lobby, color, token) => {
                self.data.do_send(RequestPlayerRemovalMessage(
                    lobby,
                    color,
                    token
                ));
            }
            WsMessage::RequestGameSubscription(lobby, token) => {
                self.data.do_send(RequestGameSubscriptionMessage(
                    lobby,
                    token,
                    recip
                ));
            }
            WsMessage::RequestGameUnsubscription(lobby, token) => {
                self.data.do_send(RequestGameUnsubscriptionMessage(
                    lobby,
                    token,
                ));
            }
            WsMessage::RequestPlayers(lobby, token) => {
                self.data.do_send(RequestPlayersMessage (
                    lobby,
                    token,
                    recip
                ));
            }
            WsMessage::RequestPlayerNameUpdate(lobby, color, token, name) => {
                self.data.do_send(RequestOpponentNameUpdateMessage (
                    lobby,
                    color,
                    token,
                    name
                ));
            }
            WsMessage::RequestMakeMove(lobby, color, token, mvpair) => {
                self.data.do_send(RequestMakeMove (
                    lobby,
                    color,
                    token,
                    mvpair
                ));
            }

            WsMessage::RequestBoardCallback(_)
            | WsMessage::RequestPlayerTokenCallback(_)
            | WsMessage::RequestPlayersCallback(_)
            | WsMessage::ConsiderRequestingBoard
            | WsMessage::ConsiderRequestingPlayers
            | WsMessage::ConsiderSubscription(_)
            | WsMessage::Hb => {
                println!("info: got unexpected ws message, ignoring:\n{:?}", message)
            }
        }
    }
}

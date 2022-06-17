use actix::{Handler, Message, Recipient};
use chessbik_commons::{Lobby, WsMessage};

use crate::websocket::InternalWsMessage;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestBoardMessage(pub Lobby, pub Recipient<InternalWsMessage>);

impl Handler<RequestBoardMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestBoardMessage(lobby, recip): RequestBoardMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(game) = self.with_game(lobby) {
            recip.do_send(InternalWsMessage(WsMessage::RequestBoardCallback(
                game.board,
            )));
        }
    }
}

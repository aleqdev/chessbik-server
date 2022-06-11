use actix::{Handler, Message, Recipient};
use chessbik_commons::{Lobby, PlayerToken};

use crate::websocket::InternalWsMessage;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestGameSubscriptionMessage(pub Lobby, pub PlayerToken, pub Recipient<InternalWsMessage>);

impl Handler<RequestGameSubscriptionMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestGameSubscriptionMessage(lobby, token, recip): RequestGameSubscriptionMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            game.subscribers.insert(token, recip);
        }
    }
}

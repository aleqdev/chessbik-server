use actix::{Handler, Message};
use chessbik_commons::{Lobby, PlayerColor, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestEngineAdditionMessage(pub Lobby, pub PlayerColor, pub PlayerToken);

impl Handler<RequestEngineAdditionMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestEngineAdditionMessage(lobby, color, token): RequestEngineAdditionMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            if let Player::None = game.players.playing(color) {
                *game.players.playing(color) = Player::Engine(token);
                game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
            }
        }
    }
}

use actix::{Handler, Message};
use chessbik_commons::{Lobby, OpponentName, PlayerColor, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestOpponentAdditionMessage(
    pub Lobby,
    pub PlayerColor,
    pub PlayerToken,
    pub OpponentName,
);

impl Handler<RequestOpponentAdditionMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestOpponentAdditionMessage(lobby, color, token, name): RequestOpponentAdditionMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            if let Player::None = game.players.playing(color) {
                *game.players.playing(color) = Player::Opponent { token, name };
                game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
            }
        }
    }
}

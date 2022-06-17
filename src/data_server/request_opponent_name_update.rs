use actix::{Handler, Message};
use chessbik_commons::{Lobby, OpponentName, PlayerColor, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestOpponentNameUpdateMessage(
    pub Lobby,
    pub PlayerColor,
    pub PlayerToken,
    pub OpponentName,
);

impl Handler<RequestOpponentNameUpdateMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestOpponentNameUpdateMessage(lobby, color, r_token, r_name): RequestOpponentNameUpdateMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            match game.players.playing(color) {
                Player::None | Player::Engine { .. } => (),
                Player::Opponent { token, name } => {
                    if *token == r_token {
                        *name = r_name;
                        game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
                    }
                }
            }
        }
    }
}

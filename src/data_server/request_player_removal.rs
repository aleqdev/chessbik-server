use actix::{Handler, Message};
use chessbik_commons::{Lobby, PlayerColor, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestPlayerRemovalMessage(pub Lobby, pub PlayerColor, pub PlayerToken);

impl Handler<RequestPlayerRemovalMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestPlayerRemovalMessage(lobby, color, r_token): RequestPlayerRemovalMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            match game.players.playing(color) {
                Player::None => {}
                Player::Opponent { token, .. } => {
                    if *token == r_token {
                        *game.players.playing(color) = Player::None;
                        game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
                    }
                }
                Player::Engine { owner, .. } => {
                    if *owner == r_token {
                        *game.players.playing(color) = Player::None;
                        //actor.
                        game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
                    }
                }
            }
        }
    }
}

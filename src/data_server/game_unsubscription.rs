use actix::{Handler, Message};
use chessbik_commons::{Lobby, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestGameUnsubscriptionMessage(pub Lobby, pub PlayerToken);

impl Handler<RequestGameUnsubscriptionMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestGameUnsubscriptionMessage(lobby, r_token): RequestGameUnsubscriptionMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            game.subscribers.remove(&r_token);

            let mut should_notify = false;

            if let Player::Opponent(ref token, _) = game.players.white {
                if *token == r_token {
                    game.players.white = Player::None;
                    should_notify = true;
                }
            }

            if let Player::Opponent(ref token, _) = game.players.black {
                if *token == r_token {
                    game.players.black = Player::None;
                    should_notify = true;
                }
            }

            if should_notify {
                game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
            }
        }
    }
}

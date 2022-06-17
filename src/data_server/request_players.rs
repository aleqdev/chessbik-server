use actix::{Handler, Message, Recipient};
use chessbik_commons::{Lobby, PlayerRecord, PlayerToken, PlayersRecord, WsMessage};

use crate::{data::Player, websocket::InternalWsMessage};

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestPlayersMessage(pub Lobby, pub PlayerToken, pub Recipient<InternalWsMessage>);

impl Handler<RequestPlayersMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestPlayersMessage(lobby, r_token, recip): RequestPlayersMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(game) = self.with_game(lobby) {
            let record = PlayersRecord {
                white: match &game.players.white {
                    Player::None => PlayerRecord::None,
                    Player::Engine { owner, .. } => {
                        PlayerRecord::Engine((r_token == *owner).into())
                    }
                    Player::Opponent { token, name } => {
                        PlayerRecord::Opponent(name.clone(), (r_token == *token).into())
                    }
                },
                black: match &game.players.black {
                    Player::None => PlayerRecord::None,
                    Player::Engine { owner, .. } => {
                        PlayerRecord::Engine((r_token == *owner).into())
                    }
                    Player::Opponent { token, name } => {
                        PlayerRecord::Opponent(name.clone(), (r_token == *token).into())
                    }
                },
            };

            recip.do_send(InternalWsMessage(WsMessage::RequestPlayersCallback(record)));
        }
    }
}

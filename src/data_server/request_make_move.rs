use actix::{Handler, Message};
use chessbik_commons::{Lobby, PlayerColor, PlayerToken, PieceMovePair, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestMakeMove(
    pub Lobby,
    pub PlayerColor,
    pub PlayerToken,
    pub PieceMovePair,
);

impl Handler<RequestMakeMove> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestMakeMove(lobby, color, r_token, mvpair @ PieceMovePair{from, mv}): RequestMakeMove,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            if let Player::Opponent(token, _) = game.players.playing(color) {
                if r_token == *token {
                    if game.board.get_available_moves(from)
                        .iter()
                        .any(|m| *m == mv) {
                        
                        Self::apply_move_unchecked(&mut game.board, mvpair);

                        game.notify_subscribers(WsMessage::ConsiderRequestingBoard);
                    }
                }
            }
        }
    }
}

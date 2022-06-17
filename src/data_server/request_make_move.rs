use actix::{Handler, Message};
use chessbik_board::{BoardStatus, PieceColor};
use chessbik_commons::{Lobby, PieceMovePair, PlayerColor, PlayerToken, WsMessage};

use crate::data::Player;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestMakeMoveMessage(
    pub Lobby,
    pub PlayerColor,
    pub PlayerToken,
    pub PieceMovePair,
);

impl Handler<RequestMakeMoveMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestMakeMoveMessage(lobby, color, r_token, mvpair @ PieceMovePair{from, mv}): RequestMakeMoveMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        if let Some(mut game) = self.with_game(lobby) {
            match game.players.playing(color) {
                Player::Opponent { token, .. } | Player::Engine { token, .. } => {
                    if r_token == *token {
                        if !match game.board.status {
                            BoardStatus::WhitesMove => color == PlayerColor::WHITE,
                            BoardStatus::BlacksMove => color == PlayerColor::BLACK,
                            BoardStatus::Mate => false,
                        } {
                            return;
                        }

                        if game
                            .board
                            .at(from)
                            .piece
                            .map_or(false, |p| !color.eq_piece_color(p.color))
                        {
                            return;
                        }

                        if game
                            .board
                            .get_available_moves(from)
                            .iter()
                            .any(|m| *m == mv)
                        {
                            game.board
                                .apply_move_unchecked(mvpair.mv, Some(mvpair.from));

                            for i in 0..54 {
                                if game.board
                                    .at(i)
                                    .piece
                                    .map_or(true, |p| color.eq_piece_color(p.color))
                                {
                                    continue;
                                }

                                if game.board.get_available_moves(i).len() > 0 {
                                    game.notify_subscribers(WsMessage::ConsiderRequestingBoard);
                                    return;
                                }
                            }

                            // no available moves found

                            if !game.board.validate(color.opposite().piece()) {
                                game.board.status = BoardStatus::Mate
                            } else {
                                game.board.status = BoardStatus::Mate //should be stalemate
                            }

                            game.notify_subscribers(WsMessage::ConsiderRequestingBoard);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

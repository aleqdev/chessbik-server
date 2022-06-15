use actix::{Actor, Context};
use chessbik_board::{Board, PieceMove, BoardStatus};
use chessbik_commons::{Lobby, PlayerToken, PieceMovePair, Cell};
use dashmap::{DashMap, mapref::one::RefMut};

use crate::data::Game;

mod create_game;
mod request_board;
mod request_engine_addition;
mod request_opponent_addition;
mod request_opponent_name_update;
mod request_player_removal;
mod request_player_token;
mod request_players;
mod game_subscription;
mod game_unsubscription;
mod request_make_move;

pub use create_game::*;
pub use request_board::*;
pub use request_engine_addition::*;
pub use request_opponent_addition::*;
pub use request_opponent_name_update::*;
pub use request_player_removal::*;
pub use request_player_token::*;
pub use request_players::*;
pub use game_subscription::*;
pub use game_unsubscription::*;
pub use request_make_move::*;

#[derive(Default)]
pub struct DataServer {
    pub games: DashMap<Lobby, Game>,
    pub tokens: DashMap<String, PlayerToken>,
}

impl DataServer {
    fn with_game(&self, lobby: Lobby) -> Option<RefMut<Lobby, Game>> {
        use chrono::Utc;

        match self.games.get_mut(&lobby) {
            Some(mut game) => {
                game.last_interaction = Utc::now();
                Some(game)
            }
            None => None,
        }
    }

    fn apply_move_unchecked(board: &mut Board<Cell>, PieceMovePair{from, mv}: PieceMovePair) {
        match mv {
            PieceMove::Slide(m) |
            PieceMove::Take(m) => {
                board.at_mut(m).piece = board.at(from).piece;
                board.at_mut(from).piece = None;
            }
            PieceMove::Rotation(rot) => {
                let pairs = chessbik_board::cube_rotations_field::get_positions(rot);

                let mut new_board = board.clone();

                for (&from, &to) in pairs[0].iter().zip(pairs[1].iter()) {
                    *new_board.at_mut(to) = *board.at(from);
                }

                *board = new_board;
            }
        }

        match board.status {
            BoardStatus::WhitesMove => board.status = BoardStatus::BlacksMove,
            BoardStatus::BlacksMove => board.status = BoardStatus::WhitesMove,
            BoardStatus::Mate => {},
        }
    }
}

impl Actor for DataServer {
    type Context = Context<Self>;
}

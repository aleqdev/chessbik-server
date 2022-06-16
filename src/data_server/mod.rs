use actix::{Actor, Context};
use chessbik_commons::{Lobby, PlayerToken};
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
}

impl Actor for DataServer {
    type Context = Context<Self>;
}

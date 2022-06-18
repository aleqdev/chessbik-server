use actix::{Actor, Context, Handler, Message, AsyncContext};
use chessbik_commons::{Lobby, PlayerToken};
use chrono::{Duration, Utc};
use dashmap::{mapref::one::RefMut, DashMap};

use crate::data::{Game, Player};

mod create_game;
mod game_subscription;
mod game_unsubscription;
mod request_board;
mod request_engine_addition;
mod request_make_move;
mod request_opponent_addition;
mod request_opponent_name_update;
mod request_player_removal;
mod request_player_token;
mod request_players;

pub use create_game::*;
pub use game_subscription::*;
pub use game_unsubscription::*;
pub use request_board::*;
pub use request_engine_addition::*;
pub use request_make_move::*;
pub use request_opponent_addition::*;
pub use request_opponent_name_update::*;
pub use request_player_removal::*;
pub use request_player_token::*;
pub use request_players::*;

#[allow(dead_code)]
#[derive(Message)]
#[rtype(result = "()")]
pub enum DataServerMessage {
    CreateGame(CreateGameMessage),
    RequestGameSubscription(RequestGameSubscriptionMessage),
    RequestGameUnsubscription(RequestGameUnsubscriptionMessage),
    RequestBoard(RequestBoardMessage),
    RequestEngineAddition(RequestEngineAdditionMessage),
    RequestMakeMove(RequestMakeMoveMessage),
    RequestOpponentAddition(RequestOpponentAdditionMessage),
    RequestOpponentNameUpdate(RequestOpponentNameUpdateMessage),
    RequestPlayerRemoval(RequestPlayerRemovalMessage),
    RequestPlayerToken(RequestPlayerTokenMessage),
    RequestPlayers(RequestPlayersMessage),
}

#[derive(Default)]
pub struct DataServer {
    pub games: DashMap<Lobby, Game>,
    pub tokens: DashMap<String, PlayerToken>,
}

impl DataServer {
    fn with_game(&self, lobby: Lobby) -> Option<RefMut<Lobby, Game>> {
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

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::minutes(5).to_std().unwrap(), |slf, _| {
            println!("Cleanup: {}", Utc::now());

            let mut to_remove = vec!();

            for game in slf.games.iter() {
                if Utc::now() - game.last_interaction > Duration::hours(12) {
                    let key = game.key().clone();
                    println!("- Removed game: {key:?} (no interaction in 12 hours)");
                    to_remove.push(key);
                } else if game.subscribers.len() == 2 {
                    if let Player::Engine { .. } = game.players.white {
                        if let Player::Engine { .. } = game.players.black {
                            let key = game.key().clone();
                            println!("- Removed game: {key:?} (2 engines with no watchers)");
                            to_remove.push(key);
                        }
                    }
                }
            }

            for key in to_remove {
                slf.games.remove(&key);
            }
        });
    }
}

impl Handler<DataServerMessage> for DataServer {
    type Result = ();

    fn handle(&mut self, msg: DataServerMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            DataServerMessage::CreateGame(m) => self.handle(m, ctx),
            DataServerMessage::RequestGameSubscription(m) => self.handle(m, ctx),
            DataServerMessage::RequestGameUnsubscription(m) => self.handle(m, ctx),
            DataServerMessage::RequestBoard(m) => self.handle(m, ctx),
            DataServerMessage::RequestEngineAddition(m) => self.handle(m, ctx),
            DataServerMessage::RequestMakeMove(m) => self.handle(m, ctx),
            DataServerMessage::RequestOpponentAddition(m) => self.handle(m, ctx),
            DataServerMessage::RequestOpponentNameUpdate(m) => self.handle(m, ctx),
            DataServerMessage::RequestPlayerRemoval(m) => self.handle(m, ctx),
            DataServerMessage::RequestPlayerToken(m) => self.handle(m, ctx),
            DataServerMessage::RequestPlayers(m) => self.handle(m, ctx),
        }
    }
}

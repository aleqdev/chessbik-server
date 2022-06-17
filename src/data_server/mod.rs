use actix::{Actor, Context, Handler, Message};
use chessbik_commons::{Lobby, PlayerToken};
use dashmap::{mapref::one::RefMut, DashMap};

use crate::data::Game;

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

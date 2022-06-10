use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{self, WebsocketContext};

use chessbik_commons::{
    Lobby, OpponentName, Player, PlayerColor, PlayerRecord, PlayerToken, PlayersRecord, WsMessage,
};
use chrono::Utc;

use crate::data::Game;

pub struct Ws {
    pub data: crate::data::DataTy,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("got message:\n{}\n", text);
                self.handle_ws_text(text, ctx);
            }
            _ => (),
        }
    }

    /*fn started(&mut self, ctx: &mut Self::Context) {
        use actix::AsyncContext;
        use std::time::Duration;

        ctx.run_interval(Duration::from_secs_f32(0.1), |_, ctx| {
            ctx.text("123 123 server");
            println!("sent message! {:?}", std::time::Instant::now());
        });
    }*/
}

impl Ws {
    fn handle_ws_text(&self, text: impl AsRef<str>, ctx: &mut WebsocketContext<Self>) {
        match serde_json::from_str::<WsMessage>(text.as_ref()) {
            Err(err) => println!(
                "error: failed to deserialize ws message, ignoring:\n{}",
                err
            ),
            Ok(message) => self.handle_ws_message(message, ctx),
        }
    }

    fn handle_ws_message(&self, message: WsMessage, ctx: &mut WebsocketContext<Self>) {
        match message {
            WsMessage::CreateGame => {
                self.handle_create_game_message(ctx);
            }
            WsMessage::RequestBoard(lobby) => {
                self.handle_request_board_message(lobby, ctx);
            }
            WsMessage::RequestPlayerToken => {
                self.handle_request_player_token_message(ctx);
            }
            WsMessage::RequestOpponentAddition(lobby, color, token, name) => {
                self.handle_request_opponent_addition_message(lobby, color, token, name);
            }
            WsMessage::RequestEngineAddition(lobby, color, token) => {
                self.handle_request_engine_addition_message(lobby, color, token);
            }
            WsMessage::RequestPlayerRemoval(lobby, color, token) => {
                self.handle_request_opponent_removal_message(lobby, color, token);
            }
            WsMessage::JoinGame(lobby) => {
                self.handle_join_game_message(lobby);
            }
            WsMessage::RequestPlayers(lobby, token) => {
                self.handle_request_players_message(lobby, token, ctx);
            }
            WsMessage::RequestPlayerNameUpdate(lobby, color, token, name) => {
                self.handle_request_player_name_update_message(lobby, color, token, name);
            }

            WsMessage::CreateGameCallback(_)
            | WsMessage::RequestBoardCallback(_)
            | WsMessage::RequestPlayerTokenCallback(_)
            | WsMessage::RequestPlayesrCallback(_)
            | WsMessage::ConsiderRequestingBoard
            | WsMessage::ConsiderRequestingPlayers => {
                println!("info: got unexpected ws message, ignoring:\n{:?}", message)
            }
        }
    }

    fn handle_join_game_message(&self, lobby: Lobby) {
        self.with_game(lobby, |game| {});
    }

    fn handle_request_player_name_update_message(
        &self,
        lobby: Lobby,
        color: PlayerColor,
        r_token: PlayerToken,
        r_name: OpponentName,
    ) {
        self.with_game(lobby, |game| match game.players.playing(color) {
            Player::None |
            Player::Engine(..) => {}
            Player::Opponent(token, name) => {
                if *token == r_token {
                    *name = r_name;
                }
            }
        });
    }

    fn handle_request_opponent_removal_message(
        &self,
        lobby: Lobby,
        color: PlayerColor,
        r_token: PlayerToken,
    ) {
        self.with_game(lobby, |game| match game.players.playing(color) {
            Player::None => {}
            Player::Engine(token) | Player::Opponent(token, _) => {
                if *token == r_token {
                    *game.players.playing(color) = Player::None;
                }
            }
        });
    }

    fn handle_request_players_message(
        &self,
        lobby: Lobby,
        r_token: PlayerToken,
        ctx: &mut WebsocketContext<Self>,
    ) {
        self.with_game(lobby, |game| {
            let record = PlayersRecord {
                white: match &game.players.white {
                    Player::None => PlayerRecord::None,
                    Player::Engine(token) => PlayerRecord::Engine((r_token == *token).into()),
                    Player::Opponent(token, name) => {
                        PlayerRecord::Opponent(name.clone(), (r_token == *token).into())
                    }
                },
                black: match &game.players.black {
                    Player::None => PlayerRecord::None,
                    Player::Engine(token) => PlayerRecord::Engine((r_token == *token).into()),
                    Player::Opponent(token, name) => {
                        PlayerRecord::Opponent(name.clone(), (r_token == *token).into())
                    }
                },
            };
            
            match serde_json::to_string(&WsMessage::RequestPlayesrCallback(record.clone())) {
                Ok(str) => ctx.text(str),
                Err(err) => println!("error: failed to serialize record ({record:?}) for [RequestPlayesrCallback] message:\n{}", err)
            };
        });
    }

    fn handle_request_opponent_addition_message(
        &self,
        lobby: Lobby,
        color: PlayerColor,
        token: PlayerToken,
        name: OpponentName,
    ) {
        self.with_game(lobby, |game| {
            if let Player::None = game.players.playing(color) {
                *game.players.playing(color) = Player::Opponent(token, name);
            }
        });
    }

    fn handle_request_engine_addition_message(
        &self,
        lobby: Lobby,
        color: PlayerColor,
        token: PlayerToken,
    ) {
        self.with_game(lobby, |game| {
            if let Player::None = game.players.playing(color) {
                *game.players.playing(color) = Player::Engine(token);
            }
        });
    }

    fn handle_request_player_token_message(&self, ctx: &mut WebsocketContext<Self>) {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let token: PlayerToken = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(14)
            .map(char::from)
            .collect::<String>()
            .into();

        match serde_json::to_string(&WsMessage::RequestPlayerTokenCallback(token.clone())) {
            Ok(str) => ctx.text(str),
            Err(err) => println!("error: failed to serialize token ({token:?}) for [RequestPlayerTokenCallback] message:\n{}", err)
        }
    }

    fn handle_create_game_message(&self, ctx: &mut WebsocketContext<Self>) {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let lobby: Lobby = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(14)
            .map(char::from)
            .collect::<String>()
            .into();

        self.data.games.insert(lobby.clone(), Game::new(Utc::now()));

        match serde_json::to_string(&WsMessage::CreateGameCallback(lobby.clone())) {
            Ok(str) => ctx.text(str),
            Err(err) => println!("error: failed to serialize lobby ({lobby:?}) for [CreateGameCallback] message:\n{}", err)
        }
    }

    fn handle_request_board_message(&self, lobby: Lobby, ctx: &mut WebsocketContext<Self>) {
        self.with_game(lobby, |game| {
            let board = game.board.clone();
            match serde_json::to_string(&WsMessage::RequestBoardCallback(board)) {
                Ok(str) => ctx.text(str),
                Err(err) => println!("error: failed to serialize board ({board:?}) for [RequestBoardCallback] message:\n{}", err)
            }
        });
    }

    fn with_game(&self, lobby: Lobby, f: impl FnOnce(&mut Game)) {
        match self.data.games.get_mut(&lobby) {
            Some(mut game) => {
                game.last_interaction = Utc::now();
                f(&mut game);
            }
            None => println!("info: game for lobby ({lobby:?}) was not found"),
        }
    }
}

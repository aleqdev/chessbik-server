use actix::{AsyncContext, Handler, Message};
use chessbik_commons::{Lobby, PlayerColor, PlayerToken, WsMessage};

use crate::{data::Player, engine_actor};

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestEngineAdditionMessage(pub Lobby, pub PlayerColor, pub PlayerToken);

impl Handler<RequestEngineAdditionMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestEngineAdditionMessage(lobby, color, token): RequestEngineAdditionMessage,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        use actix::Actor;

        if let Some(mut game) = self.with_game(lobby.clone()) {
            if let Player::None = game.players.playing(color) {
                use rand::distributions::Alphanumeric;
                use rand::Rng;

                let self_token: PlayerToken = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(14)
                    .map(char::from)
                    .collect::<String>()
                    .into();

                *game.players.playing(color) = Player::Engine {
                    token: self_token.clone(),
                    owner: token,
                    actor: engine_actor::EngineActor {
                        color,
                        lobby,
                        token: self_token,
                        data_server: ctx.address().recipient(),
                    }
                    .start(),
                };

                game.notify_subscribers(WsMessage::ConsiderRequestingPlayers);
            }
        }
    }
}

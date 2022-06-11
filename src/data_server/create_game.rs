use actix::{Handler, Message, Recipient};
use chessbik_commons::{Lobby, WsMessage};

use crate::{data::Game, websocket::InternalWsMessage};

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateGameMessage(pub Recipient<InternalWsMessage>);

impl Handler<CreateGameMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        CreateGameMessage(recip): CreateGameMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        use chrono::Utc;
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        let lobby: Lobby = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(14)
            .map(char::from)
            .collect::<String>()
            .into();

        self.games.insert(lobby.clone(), Game::new(Utc::now()));

        crate::send_to_recip(WsMessage::ConsiderSubscription(lobby), &recip)
    }
}

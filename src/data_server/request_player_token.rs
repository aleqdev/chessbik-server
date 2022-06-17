use actix::{Handler, Message, Recipient};
use chessbik_commons::{PlayerToken, WsMessage};

use crate::websocket::InternalWsMessage;

use super::DataServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestPlayerTokenMessage(pub String, pub Recipient<InternalWsMessage>);

impl Handler<RequestPlayerTokenMessage> for DataServer {
    type Result = ();

    fn handle(
        &mut self,
        RequestPlayerTokenMessage(addr, recip): RequestPlayerTokenMessage,
        _: &mut Self::Context,
    ) -> Self::Result {
        let token = match self.tokens.get(&addr) {
            Some(token) => token.value().clone(),
            None => {
                use rand::distributions::Alphanumeric;
                use rand::Rng;

                let token: PlayerToken = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(14)
                    .map(char::from)
                    .collect::<String>()
                    .into();

                self.tokens.insert(addr, token.clone());

                token.clone()
            }
        };

        recip.do_send(InternalWsMessage(WsMessage::RequestPlayerTokenCallback(
            token,
        )));
    }
}

use std::collections::HashMap;

use actix::Recipient;
use chessbik_board::Board;
use chessbik_commons::{Cell, PlayerToken, WsMessage};
use chrono::{DateTime, Utc};

use crate::websocket::InternalWsMessage;

use super::{Player, Players};

pub struct Game {
    pub players: Players,
    pub board: Board<Cell>,
    pub last_interaction: DateTime<Utc>,
    pub subscribers: HashMap<PlayerToken, Recipient<InternalWsMessage>>,
}

impl Game {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self {
            players: Players {
                white: Player::None,
                black: Player::None,
            },
            board: Default::default(),
            last_interaction: datetime,
            subscribers: Default::default(),
        }
    }

    pub fn notify_subscribers(&self, event: WsMessage) {
        for s in self.subscribers.values() {
            crate::send_to_recip(event.clone(), s);
        }
    }
}

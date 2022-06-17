use actix::Addr;
use chessbik_commons::{OpponentName, PlayerToken};

use crate::engine_actor::EngineActor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Player {
    None,
    Engine {
        token: PlayerToken,
        owner: PlayerToken,
        actor: Addr<EngineActor>,
    },
    Opponent {
        token: PlayerToken,
        name: OpponentName,
    },
}

impl Default for Player {
    fn default() -> Self {
        Self::None
    }
}

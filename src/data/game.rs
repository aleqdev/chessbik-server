use chessbik_board::Board;
use chessbik_commons::{Cell, Player, Players};
use chrono::{DateTime, Utc};

pub struct Game {
    pub players: Players,
    pub board: Board<Cell>,
    pub last_interaction: DateTime<Utc>,
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
        }
    }
}

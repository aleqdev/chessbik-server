use chessbik_commons::PlayerColor;

use super::Player;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Players {
    pub white: Player,
    pub black: Player,
}

impl Players {
    pub fn playing(&mut self, color: PlayerColor) -> &mut Player {
        match color {
            PlayerColor::WHITE => &mut self.white,
            PlayerColor::BLACK => &mut self.black,
        }
    }
}

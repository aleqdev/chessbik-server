use actix_web::web::Data;

use chessbik_commons::Lobby;
use dashmap::DashMap;

pub mod game;
pub use game::*;

#[derive(Default)]
pub struct ChessbikData {
    pub games: DashMap<Lobby, Game>,
}

impl ChessbikData {
    pub fn new_actix() -> DataTy {
        Data::new(Self::default())
    }
}

pub type DataTy = Data<ChessbikData>;

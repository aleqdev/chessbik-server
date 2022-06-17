use std::{cmp::Ordering, time::Duration};

use actix::{Actor, AsyncContext, Context, Handler, Recipient};
use chessbik_board::{Board, Eval, PieceColor, BoardStatus};
use chessbik_commons::{Cell, Lobby, PieceMovePair, PlayerColor, PlayerToken, WsMessage};
use rand::prelude::IteratorRandom;

use crate::{
    data_server::{
        DataServerMessage, RequestBoardMessage, RequestGameSubscriptionMessage,
        RequestGameUnsubscriptionMessage, RequestMakeMoveMessage,
    },
    websocket::InternalWsMessage,
};

pub struct EngineActor {
    pub color: PlayerColor,
    pub lobby: Lobby,
    pub token: PlayerToken,
    pub data_server: Recipient<DataServerMessage>,
}

impl Actor for EngineActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.data_server
            .do_send(DataServerMessage::RequestGameSubscription(
                RequestGameSubscriptionMessage(
                    self.lobby.clone(),
                    self.token.clone(),
                    ctx.address().recipient(),
                ),
            ));

        self.data_server
            .do_send(DataServerMessage::RequestBoard(RequestBoardMessage(
                self.lobby.clone(),
                ctx.address().recipient(),
            )));
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.data_server
            .do_send(DataServerMessage::RequestGameUnsubscription(
                RequestGameUnsubscriptionMessage(self.lobby.clone(), self.token.clone()),
            ))
    }
}

impl Handler<InternalWsMessage> for EngineActor {
    type Result = ();

    fn handle(&mut self, msg: InternalWsMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            WsMessage::RequestBoardCallback(board) => {
                if !match board.status {
                    BoardStatus::WhitesMove => self.color == PlayerColor::WHITE,
                    BoardStatus::BlacksMove => self.color == PlayerColor::BLACK,
                    BoardStatus::Mate => false,
                } {
                    return;
                }

                ctx.run_later(Duration::from_secs_f32(1.5), move |slf, _| {
                    let m = get_best_move(
                        &board,
                        slf.color,
                        4,
                    );
    
                    slf.data_server.do_send(DataServerMessage::RequestMakeMove(
                        RequestMakeMoveMessage(
                            slf.lobby.clone(),
                            slf.color.clone(),
                            slf.token.clone(),
                            m,
                        ),
                    ));
                });
            }

            WsMessage::ConsiderRequestingBoard => {
                self.data_server
                    .do_send(DataServerMessage::RequestBoard(RequestBoardMessage(
                        self.lobby.clone(),
                        ctx.address().recipient(),
                    )));
            }

            WsMessage::CreateGame
            | WsMessage::RequestPlayerToken
            | WsMessage::RequestPlayerTokenCallback(..)
            | WsMessage::RequestBoard(..)
            | WsMessage::RequestOpponentAddition(..)
            | WsMessage::RequestEngineAddition(..)
            | WsMessage::RequestPlayers(..)
            | WsMessage::RequestPlayersCallback(..)
            | WsMessage::RequestPlayerRemoval(..)
            | WsMessage::RequestPlayerNameUpdate(..)
            | WsMessage::RequestGameSubscription(..)
            | WsMessage::RequestGameUnsubscription(..)
            | WsMessage::ConsiderRequestingPlayers
            | WsMessage::ConsiderSubscription(..)
            | WsMessage::RequestMakeMove(..)
            | WsMessage::Hb => {}
        }
    }
}

#[derive(Debug, Clone)]
struct MoveFind {
    pub mv: PieceMovePair,
    pub eval: Eval,
}

pub fn get_best_move(
    board: &Board<Cell>,
    color: PlayerColor,
    depth: usize,
) -> PieceMovePair {
    let mut v = vec![];

    let eval_correcter = match color {
        PlayerColor::WHITE => 1.,
        PlayerColor::BLACK => -1.,
    };

    for i in 0..54 {
        if board
            .at(i)
            .piece
            .map_or(true, |p| !color.eq_piece_color(p.color))
        {
            continue;
        }

        for m in board.get_available_moves(i) {
            let mut new_board = board.clone();
            new_board.apply_move_unchecked(m, Some(i.into()));
            let e = get_best_eval_recursive(
                &new_board,
                color,
                depth - 1,
                &mut Eval(f32::NEG_INFINITY),
                &mut Eval(f32::INFINITY),
            ) * eval_correcter;

            v.push(MoveFind {
                mv: PieceMovePair {
                    from: i.into(),
                    mv: m,
                },
                eval: e,
            });
        }
    }

    v.sort_by(|a, b| a.eval.partial_cmp(&b.eval).unwrap_or(Ordering::Equal).reverse());

    let best = v[0].clone();

    v.retain(|x| x.eval == best.eval);

    v.into_iter().choose(&mut rand::thread_rng()).unwrap().mv
}

pub fn get_best_eval_recursive(
    board: &Board<Cell>,
    color: PlayerColor,
    depth: usize,
    a: &mut Eval,
    b: &mut Eval,
) -> Eval {
    use chessbik_board::GetEval;

    if depth == 0 {
        board.get_eval()
    } else {
        let next_color = match color {
            PlayerColor::WHITE => PlayerColor::BLACK,
            PlayerColor::BLACK => PlayerColor::WHITE,
        };

        if color == PlayerColor::WHITE {
            let mut e = Eval(f32::NEG_INFINITY);

            for i in 0..54 {
                if board
                    .at(i)
                    .piece
                    .map_or(true, |p| p.color != PieceColor::WHITE)
                {
                    continue;
                }

                for m in board.get_available_moves(i) {
                    let mut new_board = board.clone();
                    new_board.apply_move_unchecked(m, Some(i.into()));

                    e = Eval(e.max(*get_best_eval_recursive(
                        &new_board,
                        next_color,
                        depth - 1,
                        a,
                        b,
                    )));

                    /*if e >= *b {
                        break;
                    }

                    *a = Eval(a.max(e.0));*/
                }
            }

            e
        } else {
            let mut e = Eval(f32::INFINITY);

            for i in 0..54 {
                if board
                    .at(i)
                    .piece
                    .map_or(true, |p| p.color != PieceColor::BLACK)
                {
                    continue;
                }

                for m in board.get_available_moves(i) {
                    let mut new_board = board.clone();
                    new_board.apply_move_unchecked(m, Some(i.into()));

                    e = Eval(e.min(*get_best_eval_recursive(
                        &new_board,
                        next_color,
                        depth - 1,
                        a,
                        b,
                    )));

                    /*if e <= *a {
                        break;
                    }

                    *b = Eval(b.min(e.0));*/
                }
            }

            e
        }
    }
}

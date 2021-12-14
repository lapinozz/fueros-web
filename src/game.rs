use crate::{board::Board, player::Player};

pub struct FuerosGame {
    players: Vec<Player>,
    board: Board,
    current_player_idx: usize,
}

impl FuerosGame {
    pub fn new(players: Vec<Player>, board: Board) -> Self {
        Self {
            players,
            board,
            current_player_idx: 0,
        }
    }
}

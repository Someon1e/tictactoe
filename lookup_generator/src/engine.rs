use tictactoe::board::{bit_board::BitBoard, Board};

pub struct Engine {
    pub transposition_table: Vec<Score>,
}

const LOOKUP_SIZE: usize = 1 << 18;

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct Score(pub i8);
impl Score {
    pub const UNKNOWN: Self = Self(-i8::MAX);
    pub const WINNING: Self = Self(10);
    pub const LOSING: Self = Self(-10);
    pub const DRAWING: Self = Self(0);
}

impl Engine {
    pub fn new() -> Self {
        Self {
            transposition_table: vec![Score::UNKNOWN; LOOKUP_SIZE],
        }
    }
    pub fn search(&mut self, board: Board, x_to_move: bool) -> Score {
        let index = (board.x.0 as usize) | (board.o.0 as usize) << 9;

        let saved = self.transposition_table[index];
        if saved != Score::UNKNOWN {
            return saved;
        }

        let enemy_board = if x_to_move { board.o } else { board.x };
        if enemy_board.has_won() {
            self.transposition_table[index] = Score::LOSING;
            return Score::LOSING;
        }

        let mut best_score = Score::UNKNOWN;

        let mut not_occupied = BitBoard(!(board.x.0 | board.o.0) & BitBoard::FULL.0);
        while not_occupied != BitBoard::EMPTY {
            let place = 1 << not_occupied.pop();
            let mut new_board = board;
            if x_to_move {
                new_board.x.0 |= place;
            } else {
                new_board.o.0 |= place;
            };
            let score = -self.search(new_board, !x_to_move).0;
            if score > best_score.0 {
                best_score = Score(score);
            }
        }

        if best_score == Score::UNKNOWN {
            best_score = Score::DRAWING;
        }
        self.transposition_table[index] = best_score;
        best_score
    }
}

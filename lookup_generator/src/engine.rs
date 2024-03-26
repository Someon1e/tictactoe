use tictactoe::board::{bit_board::BitBoard, Board};

pub struct Engine {
    pub transposition_table: Vec<Score>,
}

#[derive(Clone, Copy, Debug)]
pub enum Score {
    Unknown,
    XLosing,
    Drawing,
    XWinning,
}

const LOOKUP_SIZE: usize = 1 << 18;

impl Engine {
    pub fn new() -> Self {
        Self {
            transposition_table: vec![Score::Unknown; LOOKUP_SIZE],
        }
    }
    pub fn search(&mut self, board: Board, x_to_move: bool) -> Score {
        let saved = self.transposition_table[(board.x.0 | board.o.0 << 9) as usize];
        if !matches!(saved, Score::Unknown) {
            return saved;
        }

        if board.x.has_won() {
            self.transposition_table[(board.x.0 | board.o.0 << 9) as usize] = Score::XWinning;
            return Score::XWinning;
        } else if board.o.has_won() {
            self.transposition_table[(board.x.0 | board.o.0 << 9) as usize] = Score::XLosing;
            return Score::XLosing;
        }

        let mut best_score = Score::Drawing;
        for index in 0..8 {
            if BitBoard(board.x.0 | board.o.0).get(index) {
                continue;
            }
            let mut new_board = board;
            if x_to_move {
                new_board.x.set(index);
            } else {
                new_board.o.set(index);
            };
            let score = self.search(new_board, !x_to_move);
            if (score as u8) > (best_score as u8) {
                best_score = score;
            }
        }

        self.transposition_table[(board.x.0 | board.o.0 << 9) as usize] = best_score;
        best_score
    }
}

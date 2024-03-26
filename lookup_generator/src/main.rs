#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use engine::Engine;
use tictactoe::board::{bit_board::BitBoard, Board};

use crate::engine::Score;

mod engine;

/// Generate a massive lookup table of positions to score
fn main() {
    let mut engine = Engine::new();
    engine.search(Board::EMPTY, true);
    for (position, score) in engine.transposition_table.iter().enumerate() {
        if matches!(score, Score::Unknown) {
            continue;
        };

        let x = BitBoard((position & 0b111_111_111) as u16);
        let o = BitBoard((position >> 9) as u16);
        println!("{}", Board { x, o });
        println!("{score:?}");
        println!(">>>>>><<<<<<");
    }
}

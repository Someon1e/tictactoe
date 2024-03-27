#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use engine::Engine;
use std::io::{stdout, Write};
use tictactoe::board::{bit_board::BitBoard, Board};

use crate::engine::Score;

mod engine;

/// Generate a massive lookup table of positions to score
fn main() {
    let mut engine = Engine::new();
    engine.search(Board::EMPTY, true);

    let mut stdout = stdout().lock();

    for (position, score) in engine.transposition_table.iter().enumerate() {
        if *score == Score::UNKNOWN {
            continue;
        };

        let x = BitBoard((position & 0b111_111_111) as u16);
        let o = BitBoard((position >> 9) as u16);

        let x_to_move = o.count() == x.count();

        writeln!(stdout, "{}", Board { x, o }).unwrap();
        match if x_to_move { *score } else { Score(-score.0) } {
            Score::LOSING => writeln!(stdout, "O is winning").unwrap(),
            Score::DRAWING => writeln!(stdout, "This can be drawn").unwrap(),
            Score::WINNING => writeln!(stdout, "X is winning").unwrap(),
            _ => unreachable!(),
        }
        writeln!(stdout, ">>>>>><<<<<<").unwrap();
        writeln!(stdout).unwrap();
    }
}

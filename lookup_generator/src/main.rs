#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use engine::Engine;
use std::{
    fs,
    io::{stdout, Write},
};
use tictactoe::board::{bit_board::BitBoard, Board};

use crate::engine::Score;

mod engine;

/// Generate a massive lookup table of positions to score
fn main() {
    let mut engine = Engine::new();
    engine.search(Board::EMPTY, true);

    let mut stdout = stdout().lock();

    // Pretty output of board and score
    for (position, score) in engine.transposition_table.iter().enumerate() {
        if *score == Score::UNKNOWN {
            continue;
        };

        #[allow(clippy::cast_possible_truncation)]
        let (x, o) = {
            (
                BitBoard::new((position & 0b111_111_111) as u16),
                BitBoard::new((position >> 9) as u16),
            )
        };

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

    // Generate rust match expression
    let mut matcher = String::new();
    matcher.push_str("match position {\n");

    let mut drawing = String::new();
    let mut winning = String::new();
    let mut losing = String::new();

    for (position, score) in engine.transposition_table.iter().enumerate() {
        if *score == Score::UNKNOWN {
            continue;
        };

        #[allow(clippy::cast_possible_truncation)]
        let (x, o) = {
            (
                BitBoard::new((position & 0b111_111_111) as u16),
                BitBoard::new((position >> 9) as u16),
            )
        };

        let x_to_move = o.count() == x.count();
        let text = &format!("{position:#04x} | ");
        match if x_to_move { *score } else { Score(-score.0) } {
            Score::LOSING => losing.push_str(text),
            Score::DRAWING => drawing.push_str(text),
            Score::WINNING => winning.push_str(text),
            _ => unreachable!(),
        }
    }
    matcher.push('\t');
    matcher.push_str(&drawing[..drawing.len() - 2]);
    matcher.push_str("=> 0,\n");

    matcher.push('\t');
    matcher.push_str(&winning[..winning.len() - 2]);
    matcher.push_str("=> 1,\n");

    matcher.push('\t');
    matcher.push_str(&losing[..losing.len() - 2]);
    matcher.push_str("=> -1,\n");

    matcher.push('\t');
    matcher.push_str("_ => unreachable!(),\n");

    matcher.push_str("};");
    fs::write("match.rs", matcher).unwrap();
}

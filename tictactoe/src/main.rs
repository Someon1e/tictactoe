#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use tictactoe::board::Board;
use std::io::Write;
use std::io::{stdin, stdout, BufRead, StdinLock, StdoutLock};

struct Game<'a> {
    stdin: StdinLock<'a>,
    stdout: StdoutLock<'a>,
    input: String,
    board: Board,
    x_to_move: bool,
}

impl<'a> Game<'a> {
    pub fn read_input(&mut self) -> Option<(u8, u8)> {
        self.stdin.read_line(&mut self.input).unwrap();

        let mut chars = self.input.chars();

        let column = match chars.nth(0) {
            Some('a') => 0,
            Some('b') => 1,
            Some('c') => 2,
            _ => return None,
        };

        let row = match chars.nth(0) {
            Some('1') => 0,
            Some('2') => 1,
            Some('3') => 2,
            _ => return None,
        };

        Some((column, row))
    }
    pub fn run(&mut self) {
        loop {
            writeln!(self.stdout, "{}", self.board).unwrap();
            let Some((column, row)) = self.read_input() else {
                continue;
            };

            let bit_board = if self.x_to_move {
                &mut self.board.x
            } else {
                &mut self.board.o
            };
            if bit_board.get(row * 3 + column) {
                writeln!(self.stdout, "Occupied").unwrap();
            } else {
                bit_board.set(row * 3 + column);
                if bit_board.has_won() {
                    println!("{} wins!", if self.x_to_move {"x"} else {"o"});
                    break;
                }
                self.x_to_move = !self.x_to_move;
            }

            self.input.clear();
        }
    }
}

fn main() {
    let mut game = Game {
        stdin: stdin().lock(),
        stdout: stdout().lock(),
        input: String::new(),
        board: Board::EMPTY,
        x_to_move: true
    };

    game.run();
}

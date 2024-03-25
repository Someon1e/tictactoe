use core::fmt::Display;
use core::fmt::Formatter;

use crate::bit_board::BitBoard;

/// Board uses two `BitBoards`: one for X, one for O
pub struct Board {
    pub x: BitBoard,
    pub o: BitBoard,

    pub x_to_move: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for row in (0..3).rev() {
            match row {
                0 => write!(f, "1 |")?,
                1 => write!(f, "2 |")?,
                2 => write!(f, "3 |")?,
                _ => unreachable!(),
            };
            for column in 0..3 {
                if self.x.get(row * 3 + column) {
                    write!(f, "X")?;
                } else if self.o.get(row * 3 + column) {
                    write!(f, "O")?;
                } else {
                    write!(f, "-")?;
                };
                write!(f, "|")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "   a b c")?;
        Ok(())
    }
}

impl Board {
    pub const EMPTY: Self = Self {
        x_to_move: true,
        x: BitBoard::EMPTY,
        o: BitBoard::EMPTY,
    };
}

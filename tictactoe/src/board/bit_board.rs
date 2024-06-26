use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

/// `BitBoard` is a u16, but only 9 bits are used.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(u16);

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for row in (0..3).rev() {
            for column in 0..3 {
                if self.get(row * 3 + column) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
                if column != 2 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

macro_rules! board {
    ($top:tt, $middle:tt, $bottom:tt) => {
        Self(($top << 6) | ($middle << 3) | ($bottom << 0))
    };
}

impl BitBoard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0b111_111_111);

    pub const BOTTOM_ROW: Self = Self(0b111);
    pub const MIDDLE_ROW: Self = Self(0b111 << 3);
    pub const TOP_ROW: Self = Self(0b111 << 6);

    #[rustfmt::skip]
    pub const LEFT_COLUMN: Self = board!(
        0b100,
        0b100,
        0b100
    );
    #[rustfmt::skip]
    pub const MIDDLE_COLUMN: Self = board!(
        0b010,
        0b010,
        0b010
    );
    #[rustfmt::skip]
    pub const RIGHT_COLUMN: Self = board!(
        0b001,
        0b001,
        0b001
    );

    #[rustfmt::skip]
    pub const TOP_RIGHT_DIAGONAL: Self = board!(
        0b001,
        0b010,
        0b100
    );
    #[rustfmt::skip]
    pub const TOP_LEFT_DIAGONAL: Self = board!(
        0b100,
        0b010,
        0b001
    );

    #[must_use]
    pub const fn new(bits: u16) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn as_u32(&self) -> u32 {
        self.0 as u32
    }

    #[must_use]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    #[must_use]
    pub const fn get(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
    }

    #[must_use]
    pub const fn contains(&self, bit_board: &Self) -> bool {
        (self.0 & bit_board.0) == bit_board.0
    }

    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn first(&self) -> u32 {
        self.0.trailing_zeros()
    }

    #[must_use]
    pub fn pop(&mut self) -> u32 {
        let index = self.first();
        self.0 &= self.0 - 1;
        index
    }

    #[must_use]
    pub const fn has_won(&self) -> bool {
        self.contains(&Self::BOTTOM_ROW)
            || self.contains(&Self::MIDDLE_ROW)
            || self.contains(&Self::TOP_ROW)
            || self.contains(&Self::LEFT_COLUMN)
            || self.contains(&Self::MIDDLE_COLUMN)
            || self.contains(&Self::RIGHT_COLUMN)
            || self.contains(&Self::TOP_LEFT_DIAGONAL)
            || self.contains(&Self::TOP_RIGHT_DIAGONAL)
    }
}

macro_rules! implement_op {
    ($op:ident, $name:ident, $operator:tt) => {
        impl $op<BitBoard> for BitBoard {
            type Output = BitBoard;

            fn $name(self, rhs: BitBoard) -> Self::Output {
               Self(self.0 $operator rhs.0)
            }
        }
    };
}
macro_rules! implement_assign_op {
    ($op:ident, $name:ident, $operator:tt) => {
        impl $op<BitBoard> for BitBoard {
            fn $name(&mut self, rhs: Self) {
                *self = Self(self.0 $operator rhs.0)
            }
        }
    };
}
implement_op!(BitOr, bitor, |);
implement_assign_op!(BitOrAssign, bitor_assign, |);

implement_op!(BitAnd, bitand, &);
implement_assign_op!(BitAndAssign, bitand_assign, &);

macro_rules! shift {
    ($op:ident, $name:ident, $operator:tt) => {
        impl $op<u8> for BitBoard {
            type Output = BitBoard;

            fn $name(self, rhs: u8) -> Self::Output {
               Self(self.0 $operator rhs)
            }
        }
    };
}

shift!(Shl, shl, <<);
shift!(Shr, shr, >>);

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

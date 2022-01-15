use crate::direction::Direction;
use crate::square::Square;
use crate::{file::File, rank::Rank};
use std::{fmt, ops::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);
pub const UNIVERSE: BitBoard = BitBoard(!0);

impl BitBoard {
    pub fn new(b: u64) -> Self {
        BitBoard(b)
    }

    pub fn from_square(sq: Square) -> Self {
        BitBoard(1u64.wrapping_shl(sq.index() as u32))
    }

    pub fn from_file(file: File) -> Self {
        match file {
            File::A => BitBoard(0x0101010101010101),
            File::B => BitBoard(0x0202020202020202),
            File::C => BitBoard(0x0404040404040404),
            File::D => BitBoard(0x0808080808080808),
            File::E => BitBoard(0x1010101010101010),
            File::F => BitBoard(0x2020202020202020),
            File::G => BitBoard(0x4040404040404040),
            File::H => BitBoard(0x8080808080808080),
        }
    }

    pub fn from_rank(rank: Rank) -> Self {
        match rank {
            Rank::Rank1 => BitBoard(0x00000000000000FF),
            Rank::Rank2 => BitBoard(0x000000000000FF00),
            Rank::Rank3 => BitBoard(0x0000000000FF0000),
            Rank::Rank4 => BitBoard(0x00000000FF000000),
            Rank::Rank5 => BitBoard(0x000000FF00000000),
            Rank::Rank6 => BitBoard(0x0000FF0000000000),
            Rank::Rank7 => BitBoard(0x00FF000000000000),
            Rank::Rank8 => BitBoard(0xFF00000000000000),
        }
    }

    pub fn to_square(self) -> Option<Square> {
        Square::from_index(self.0.trailing_zeros() as usize)
    }

    pub fn swap(self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    pub fn up(self) -> BitBoard {
        BitBoard(self.0.wrapping_shl(8))
    }

    pub fn down(self) -> BitBoard {
        BitBoard(self.0.wrapping_shr(8))
    }

    pub fn left(self) -> BitBoard {
        BitBoard(self.0.wrapping_shr(1) & 0x7F7F7F7F7F7F7F7Fu64)
    }

    pub fn right(self) -> BitBoard {
        BitBoard(self.0.wrapping_shl(1) & 0xFEFEFEFEFEFEFEFEu64)
    }

    pub fn shift(self, direction: Direction) -> BitBoard {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),

            Direction::UpLeft => self.up().left(),
            Direction::UpRight => self.up().right(),
            Direction::DownLeft => self.down().left(),
            Direction::DownRight => self.down().right(),
        }
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let sq = self.to_square();
            self.0 &= self.0 - 1;
            sq
        }
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..64 {
            let i = i ^ 56;
            if (self.0 & (1 << i)) == (1 << i) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if i % 8 == 7 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 ^= rhs.0;
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        self.0 &= rhs.0;
    }
}

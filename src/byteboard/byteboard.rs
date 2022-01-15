use std::{fmt, intrinsics::transmute};

use crate::{
    bitboard::{BitBoard, EMPTY},
    byteboard::bytesquare::*,
    square::Square,
};

#[macro_export]
macro_rules! sq {
    ($s:ident) => {
        Square::$s as usize
    };
}

#[macro_export]
macro_rules! co {
    ($s:expr) => {
        format!(
            "{}{}",
            (($s % 8) as u8 + 'a' as u8) as char,
            (($s / 8) as u8 + '1' as u8) as char
        )
    };
}

#[macro_export]
macro_rules! bs {
    () => {
        ByteSquare::from_u8(0)
    };
    ($color:ident $piece:ident) => {
        ByteSquare::new(true, ByteColor::$color, PieceType::$piece)
    };
}

#[derive(Clone, Copy)]
pub struct ByteBoard {
    squares: [ByteSquare; 64],
}

impl ByteBoard {
    pub fn new() -> Self {
        ByteBoard {
            squares: [ByteSquare::default(); 64],
        }
    }

    pub fn startpos() -> Self {
        ByteBoard::from_u64_octet([
            0x90a098808898a090,
            0xa8a8a8a8a8a8a8a8,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0xe8e8e8e8e8e8e8e8,
            0xd0e0d8c0c8d8e0d0,
        ])
    }

    pub fn invert(&mut self) {
        unsafe {
            let tmp: &mut [u64; 8] = transmute(&mut self.squares);
            for i in 0..4 {
                tmp[i] ^= 0x4040404040404040; // color flip
                tmp.swap(i, i ^ 7);
                tmp[i] ^= 0x4040404040404040; // color flip
            }
        }
    }

    fn clear_last_move(&mut self) {
        for i in 0..64 {
            self.squares[i].set_last_move(false);
        }
    }

    pub fn make_move(&mut self, from: usize, to: usize) {
        if from == to || self.squares[from].is_empty() {
            return;
        }
        self.squares[to] = self.squares[from];
        self.squares[from] = ByteSquare::new_empty();
        self.clear_last_move();
        self.squares[to].set_last_move(true);
    }

    pub fn get_turn(&self) -> ByteColor {
        for square in self.squares {
            if square.occupied() && square.last_move() {
                return match square.color() {
                    ByteColor::White => ByteColor::Black,
                    ByteColor::Black => ByteColor::White,
                };
            }
        }
        ByteColor::White // Default to white for the first move
    }

    pub fn to_u64_octet(&self) -> [u64; 8] {
        let mut octet = [0u64; 8];
        unsafe {
            for (i, chunk) in self.squares.chunks(8).enumerate() {
                octet[i] = u64::from_be_bytes([
                    transmute(chunk[0]),
                    transmute(chunk[1]),
                    transmute(chunk[2]),
                    transmute(chunk[3]),
                    transmute(chunk[4]),
                    transmute(chunk[5]),
                    transmute(chunk[6]),
                    transmute(chunk[7]),
                ]);
            }
        }
        octet
    }

    pub fn to_u128_quad(&self) -> [u128; 4] {
        let mut quad = [0u128; 4];
        unsafe {
            for (i, chunk) in self.squares.chunks(16).enumerate() {
                quad[i] = u128::from_be_bytes([
                    transmute(chunk[0]),
                    transmute(chunk[1]),
                    transmute(chunk[2]),
                    transmute(chunk[3]),
                    transmute(chunk[4]),
                    transmute(chunk[5]),
                    transmute(chunk[6]),
                    transmute(chunk[7]),
                    transmute(chunk[8]),
                    transmute(chunk[9]),
                    transmute(chunk[10]),
                    transmute(chunk[11]),
                    transmute(chunk[12]),
                    transmute(chunk[13]),
                    transmute(chunk[14]),
                    transmute(chunk[15]),
                ]);
            }
        }
        quad
    }

    pub fn from_u128_quad(quad: [u128; 4]) -> Self {
        let mut squares = [ByteSquare::default(); 64];
        for (i, chunk) in quad.chunks(4).enumerate() {
            for j in 0..4 {
                squares[i * 4 + j] = ByteSquare::from_u8((chunk[j] >> (j << 3)) as u8);
            }
        }
        ByteBoard { squares }
    }

    pub fn from_u64_octet(octet: [u64; 8]) -> Self {
        let mut squares = [ByteSquare::default(); 64];

        for (i, chunk) in octet.iter().enumerate() {
            for j in 0..8 {
                squares[i * 8 + j] = ByteSquare::from_u8((*chunk >> (j << 3)) as u8);
            }
        }

        ByteBoard { squares }
    }

    pub fn set(&mut self, index: usize, square: ByteSquare) {
        self.squares[index] = square;
    }

    pub fn get(&self, index: usize) -> ByteSquare {
        self.squares[index]
    }

    pub fn occupied(&self) -> impl Iterator<Item = usize> + '_ {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, square)| square.occupied())
            .map(|(index, _)| index)
    }

    pub fn bitboard(&self) -> BitBoard {
        let mut bitboard = EMPTY;
        for square in self.occupied() {
            bitboard |= BitBoard::from_square(Square::from_index(square).unwrap());
        }
        bitboard
    }
}

impl fmt::Display for ByteBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row = 0;
        write!(f, "\n  ---------------------------------\n8 |")?;
        for i in 0..64 {
            let square = &self.squares[i % 8 + 56 - (i / 8) * 8];
            write!(f, " {} ", square)?;
            // write!(f, " {} ", i % 8 + 56 - (i / 8) * 8)?;
            if row % 8 == 7 {
                if row == 63 {
                    // Last row (bottom)
                    write!(
                        f,
                        "|\n  ---------------------------------\n    a   b   c   d   e   f   g   h\n"
                    )?;
                } else {
                    write!(
                        f,
                        "|\n  ---------------------------------\n{} |",
                        8 - (row + 1) / 8
                    )?;
                }
            } else {
                write!(f, "|")?;
            }
            row += 1;
        }
        Ok(())
    }
}

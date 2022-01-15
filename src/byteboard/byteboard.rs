use std::{fmt, intrinsics::transmute};

const OCCUPIED_MASK: u8 = 0b10000000;

#[derive(Debug)]
pub enum ByteColor {
    White = 0b00000000,
    Black = 0b01000000,
}
const COLOR_MASK: u8 = 0b01000000;

enum PieceType {
    King = 0b00000000,
    Queen = 0b00001000,
    Rook = 0b00010000,
    Bishop = 0b00011000,
    Knight = 0b00100000,
    Pawn = 0b00101000,
}
const PIECE_MASK: u8 = 0b00111000;

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                PieceType::Pawn => "p",
                PieceType::Knight => "n",
                PieceType::Bishop => "b",
                PieceType::Rook => "r",
                PieceType::Queen => "q",
                PieceType::King => "k",
            }
        )
    }
}

const KING_OR_ROOK_MOVED_FLAG_MASK: u8 = 0b00000100;
const PAWN_EN_PASSANT_FLAG_MASK: u8 = 0b00000100;
const LAST_MOVE_FLAG_MASK: u8 = 0b00000010;

#[derive(Copy, Clone)]
struct ByteSquare(u8);

impl ByteSquare {
    pub fn occupied(&self) -> bool {
        (self.0 & OCCUPIED_MASK) != 0
    }

    pub fn color(&self) -> ByteColor {
        unsafe { std::mem::transmute(self.0 & COLOR_MASK) }
    }

    pub fn piece_type(&self) -> PieceType {
        unsafe { std::mem::transmute(self.0 & PIECE_MASK) }
    }

    pub fn king_or_rook_moved(&self) -> bool {
        (self.0 & KING_OR_ROOK_MOVED_FLAG_MASK) != 0
    }

    pub fn pawn_en_passant(&self) -> bool {
        (self.0 & PAWN_EN_PASSANT_FLAG_MASK) != 0
    }

    pub fn last_move(&self) -> bool {
        (self.0 & LAST_MOVE_FLAG_MASK) != 0
    }

    pub fn set_occupied(&mut self, occupied: bool) {
        if occupied {
            self.0 |= OCCUPIED_MASK;
        } else {
            self.0 &= !OCCUPIED_MASK;
        }
    }

    pub fn set_color(&mut self, color: ByteColor) {
        self.0 &= !COLOR_MASK;
        self.0 |= color as u8;
    }

    pub fn invert_color(&mut self) {
        self.0 ^= COLOR_MASK;
    }

    pub fn set_piece_type(&mut self, piece_type: PieceType) {
        self.0 &= !PIECE_MASK;
        self.0 |= piece_type as u8;
    }

    pub fn set_king_or_rook_moved(&mut self, moved: bool) {
        if moved {
            self.0 |= KING_OR_ROOK_MOVED_FLAG_MASK;
        } else {
            self.0 &= !KING_OR_ROOK_MOVED_FLAG_MASK;
        }
    }

    pub fn set_pawn_en_passant(&mut self, en_passant: bool) {
        if en_passant {
            self.0 |= PAWN_EN_PASSANT_FLAG_MASK;
        } else {
            self.0 &= !PAWN_EN_PASSANT_FLAG_MASK;
        }
    }

    pub fn set_last_move(&mut self, last_move: bool) {
        if last_move {
            self.0 |= LAST_MOVE_FLAG_MASK;
        } else {
            self.0 &= !LAST_MOVE_FLAG_MASK;
        }
    }

    pub fn from_u8(byte: u8) -> ByteSquare {
        ByteSquare(byte)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn new(occupied: bool, color: ByteColor, piece_type: PieceType) -> ByteSquare {
        let mut square = ByteSquare(0);
        square.set_occupied(occupied);
        square.set_color(color);
        square.set_piece_type(piece_type);
        square
    }

    pub fn new_empty() -> ByteSquare {
        ByteSquare(0)
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Default for ByteSquare {
    fn default() -> ByteSquare {
        ByteSquare::new_empty()
    }
}

impl fmt::Display for ByteSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.occupied() {
            write!(
                f,
                "{}",
                match self.color() {
                    ByteColor::White => self.piece_type().to_string().to_uppercase(),
                    ByteColor::Black => self.piece_type().to_string().to_lowercase(),
                }
            )
        } else {
            write!(f, " ")
        }
    }
}

macro_rules! sq {
    () => {
        ByteSquare::new_empty()
    };
    ($color:ident $piece_type:ident) => {
        ByteSquare::new(true, ByteColor::$color, PieceType::$piece_type)
    };
}

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

    pub fn coord_to_index(coord: &str) -> usize {
        let file = coord.chars().nth(0).unwrap() as u8 - 'a' as u8;
        let rank = coord.chars().nth(1).unwrap() as u8 - '1' as u8;
        (rank * 8 + file) as usize
    }

    pub fn index_to_coord(index: usize) -> String {
        let file = (index % 8) as u8 + 'a' as u8;
        let rank = (index / 8) as u8 + '1' as u8;
        format!("{}{}", file as char, rank as char)
    }

    fn clear_last_move(&mut self) {
        for i in 0..64 {
            self.squares[i].set_last_move(false);
        }
    }

    pub fn make_move(&mut self, from: &str, to: &str) {
        let from_index = ByteBoard::coord_to_index(from);
        let to_index = ByteBoard::coord_to_index(to);
        if from_index == to_index {
            return;
        }
        if self.squares[from_index].is_empty() {
            return;
        }
        self.squares[to_index] = self.squares[from_index];
        self.squares[from_index] = ByteSquare::new_empty();
        self.clear_last_move();
        self.squares[to_index].set_last_move(true);
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

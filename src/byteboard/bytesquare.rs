use core::fmt;

const OCCUPIED_MASK: u8 = 0b10000000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ByteColor {
    White = 0b00000000,
    Black = 0b01000000,
}
const COLOR_MASK: u8 = 0b01000000;

#[derive(Debug)]
pub enum PieceType {
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
pub struct ByteSquare(u8);

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

    pub fn is_empty(&self) -> bool {
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

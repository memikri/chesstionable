#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    // maybe WhitePawn in future???
    // coming back on this the code actually seems pretty nice, let's hope performance is good as well
    // although if we're gonna have arrays indexed by piece it'd be really nice to just do Piece as usize
    // maybe a conversion macro? or function to convert to usize?
}

pub const WHITE_PAWN: Piece = Piece::Pawn(Color::White);
pub const BLACK_PAWN: Piece = Piece::Pawn(Color::Black);
pub const WHITE_KNIGHT: Piece = Piece::Knight(Color::White);
pub const BLACK_KNIGHT: Piece = Piece::Knight(Color::Black);
pub const WHITE_BISHOP: Piece = Piece::Bishop(Color::White);
pub const BLACK_BISHOP: Piece = Piece::Bishop(Color::Black);
pub const WHITE_ROOK: Piece = Piece::Rook(Color::White);
pub const BLACK_ROOK: Piece = Piece::Rook(Color::Black);
pub const WHITE_QUEEN: Piece = Piece::Queen(Color::White);
pub const BLACK_QUEEN: Piece = Piece::Queen(Color::Black);
pub const WHITE_KING: Piece = Piece::King(Color::White);
pub const BLACK_KING: Piece = Piece::King(Color::Black);

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        match piece_type {
            PieceType::Pawn => Piece::Pawn(color),
            PieceType::Knight => Piece::Knight(color),
            PieceType::Bishop => Piece::Bishop(color),
            PieceType::Rook => Piece::Rook(color),
            PieceType::Queen => Piece::Queen(color),
            PieceType::King => Piece::King(color),
        }
    }

    pub fn piece_type(&self) -> PieceType {
        match *self {
            Piece::Pawn(_) => PieceType::Pawn,
            Piece::Knight(_) => PieceType::Knight,
            Piece::Bishop(_) => PieceType::Bishop,
            Piece::Rook(_) => PieceType::Rook,
            Piece::Queen(_) => PieceType::Queen,
            Piece::King(_) => PieceType::King,
        }
    }

    pub fn color(&self) -> Color {
        match *self {
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Piece::Pawn(Color::White) => "P",
            Piece::Pawn(Color::Black) => "p",
            Piece::Knight(Color::White) => "N",
            Piece::Knight(Color::Black) => "n",
            Piece::Bishop(Color::White) => "B",
            Piece::Bishop(Color::Black) => "b",
            Piece::Rook(Color::White) => "R",
            Piece::Rook(Color::Black) => "r",
            Piece::Queen(Color::White) => "Q",
            Piece::Queen(Color::Black) => "q",
            Piece::King(Color::White) => "K",
            Piece::King(Color::Black) => "k",
        }
    }
}

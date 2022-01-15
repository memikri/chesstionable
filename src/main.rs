mod bitboard;
mod bitboard_moves;
mod byteboard;
mod direction;
mod errors;
mod fen;
mod file;
mod piece;
mod rank;
mod square;
mod state;
mod utils;

use square::Square;

use crate::byteboard::{
    byteboard::ByteBoard,
    bytesquare::{ByteColor, ByteSquare, PieceType},
};
// use bitboard::*;
// use bitboard_moves::*;
// use square::*;
// use state::State;

fn main() {
    let mut b = ByteBoard::startpos();
    // let mut b = ByteBoard::new();
    // b.set(sq!(A1), bs!(White King));
    // b.set(sq!(B2), bs!(White Pawn));
    for m in b.moves() {
        println!("{}{}{}", m.from, if m.capture { "x" } else { "-" }, m.to);
    }
    // for s in b.ray(sq!(C3), 7).chain(b.ray(sq!(C3), 9)) {
    //     println!("{}", co!(s));
    //     // b.set(s, bs!(White Pawn));
    // }

    // // let mut b = ByteBoard::startpos();

    // // b.make_move("g1", "f3");
    // // b.invert();

    // println!("{}", b);
}

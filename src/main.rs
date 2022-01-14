mod bitboard;
mod bitboard_moves;
mod direction;
mod errors;
mod fen;
mod file;
mod piece;
mod rank;
mod square;
mod state;
mod utils;

use bitboard::*;
use bitboard_moves::*;
use square::*;
use state::State;

// this is turning into enterprise code very quickly :kekwait:

fn main() {
    // let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
    // let state = State::from_fen(fen).unwrap();
    let state = State::default();
    println!("{:?}", state);
    println!("{:?}", state.to_fen()); // TODO: this is rotated 90 degrees lmao

    let pext_table = PextTable::new();
    let bb = pext_table.queen_moves(EMPTY, Square::F5);
    println!("{}", bb);
}

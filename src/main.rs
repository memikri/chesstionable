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

use crate::byteboard::byteboard::ByteBoard;
// use bitboard::*;
// use bitboard_moves::*;
// use square::*;
// use state::State;

fn main() {
    let mut byteboard = ByteBoard::startpos();

    loop {
        println!("{}\n{:?} to move", byteboard, byteboard.get_turn());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let from = input.get(0..2).unwrap();
        let to = input.get(2..4).unwrap();

        byteboard.make_move(from, to);
    }
}

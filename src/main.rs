mod fen;
mod piece;
mod square;
mod state;

use state::State;

// this is turning into enterprise code very quickly :kekwait:

fn main() {
    let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50"; // we need to handle both of the move counts
    let state = State::from_fen(fen).unwrap();
    // let state = State::new();
    println!("{:?}", state);
    println!("{:?}", state.to_fen());
}

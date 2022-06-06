mod engine;

use engine::parser::parse;
use engine::structs::{Piece, PieceType, Color, BoardMap, GameState};
use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = &mut fs::read_to_string("./src/static/initial.fen")
        .expect("Something went wrong reading the file");

    println!("{}", contents);

    parse(contents);

    println!("{}", contents);

    let king = Piece {
        piece_type: PieceType::King,
        color: Color::White,
    };

    let board: &mut BoardMap = &mut HashMap::new();

    board.insert((2, 2), king);

    let game_state = &mut GameState {
        board: board,
    };

    println!("{:?}", game_state.board.get(&(2, 2)));
}

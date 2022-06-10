mod engine;

use crate::engine::structs::enums::Position;
use std::collections::HashSet;
use crate::engine::move_generator::generate_moves;
use crate::engine::structs::piece::Piece;
use crate::engine::structs::enums::{PieceType, Color};
use engine::structs::game_state::GameState;
use engine::parser::parse;
use engine::utils::std_pos_to_couple;
use engine::renderer::render_board;
use std::{thread, time, fs, io};

fn main() {
    let contents = &mut fs::read_to_string("./src/static/initial.fen")
        .expect("Something went wrong reading the file");

    let contents = &mut contents.trim().to_string();

    let game_state = &mut GameState::new();

    parse(contents, game_state);

    game_state.board.insert(std_pos_to_couple(String::from("e4")), Piece::new(PieceType::Pawn, Color::White));
    game_state.board.insert(std_pos_to_couple(String::from("d5")), Piece::new(PieceType::Pawn, Color::Black));
    game_state.board.insert(std_pos_to_couple(String::from("f5")), Piece::new(PieceType::Pawn, Color::White));

    loop {
        // clear_view!();
        render_board(game_state);

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");
        let split: &mut Vec<String> = &mut s.trim().split(":").map(str::to_string).collect();

        if split.len() != 2 {
            println!("Invalid move!");
            let second = time::Duration::from_millis(1000);

            thread::sleep(second);
            continue;
        }

        let from = split.remove(0);
        let from = std_pos_to_couple(from);
        let piece = game_state.board.get(&from).unwrap().clone();

        let moves = generate_moves(game_state, piece, from);

        for i in 0..8 {
            for j in 0..8 {
                let hash: HashSet<Position> = HashSet::from_iter(moves.iter().cloned());
                if hash.contains(&(i, j)) {
                    print!("X");
                } else {
                    print!("O");
                }
            }
            println!("");
        }
    }
}

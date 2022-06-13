mod engine;

use engine::structs::game_state::GameState;
use engine::parser::{parse, parse_move};
use engine::renderer::render_board;
use engine::state_manager::execute_move;
use std::{fs, io};

fn main() {
    let contents = &mut fs::read_to_string("./src/static/initial.fen")
        .expect("Something went wrong reading the file");

    let contents = &mut contents.trim().to_string();

    let game_state = &mut GameState::new();

    parse(contents, game_state);

    loop {
        // clear_view!();
        render_board(game_state);

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");

        let (from, to) = match parse_move(s) {
            Ok(t) => t,
            Err(err) => {
                println!("Error while parsing: {}", err);
                continue;
            },
        };
 
        if let Err(err) = execute_move(game_state, from, to) {
            println!("Error while executing the move: {}", err);
        }
    }
}

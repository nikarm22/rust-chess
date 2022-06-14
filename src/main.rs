mod engine;

use engine::structs::game_state::GameState;
use engine::parser::{parse, parse_move};
use engine::utils::read_ln;
use engine::renderer::render_board;
use std::fs;

fn main() {
    let contents = &mut fs::read_to_string("./src/static/initial.fen")
        .expect("Something went wrong reading the file");

    let contents = &mut contents.trim().to_string();

    let game_state = &mut GameState::new();

    parse(contents, game_state);

    // FIXME: Implement castlign and promotion
    // FIXME: Checkmate implementation
    // TODO: add to fen_string functionality
    // TODO: add short position syntax (Disambiguate :( )
    // TODO: Basic sfml integration
    // TODO: UI, Click, highlight
    // TODO: Standardized API
    // TODO: Custom evaluation and bot
    // TODO: Talking with stockfish
    // TODO: Menu audio etc.
    // TODO: Undo move.
    loop {
        clear_view!();
        render_board(game_state);

        let move_str = read_ln();
        let (from, to) = match parse_move(move_str) {
            Ok(t) => t,
            Err(err) => {
                println!("Error while parsing: {}", err);
                continue;
            },
        };
 
        if let Err(err) = game_state.execute_move(from, to) {
            println!("Error while executing the move: {}", err);
        }
    }
}

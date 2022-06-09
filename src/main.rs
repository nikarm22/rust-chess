mod engine;

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

    loop {
        clear_view!();
        render_board(game_state);

        

        // let mut s = String::new();
        // io::stdin().read_line(&mut s).expect("Did not enter a correct string");        
        // let split: &mut Vec<String> = &mut s.trim().split(":").map(str::to_string).collect();

        // if split.len() != 2 {
        //     println!("Invalid move!");
        //     let second = time::Duration::from_millis(1000);
            
        //     thread::sleep(second);
        //     continue;
        // }

        // let from = split.remove(0);
        // let to = split.remove(0);
        // let from = std_pos_to_couple(from);
        // let to = std_pos_to_couple(to);

        // let active_piece = game_state.board.remove(&from);
        // match active_piece {
        //     Some(piece) => {game_state.board.insert(to, piece);},
        //     None => {},
        // };
    }
}

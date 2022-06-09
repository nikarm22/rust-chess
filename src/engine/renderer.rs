use super::structs::game_state::GameState;
use super::structs::enums::Color;
use colored::Colorize;

pub fn render_board (game_state: &mut GameState) {
    println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
    println!("| | | A | B | C | D | E | F | G | H | | |");
    println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
    println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
    for i in 0..8 {
        print!( "|{}| |", 8 - i);
        for j in 0..8 {
            match game_state.board.get(&(i, j)) {
                Some(piece) => {
                    let value = Colorize::bold(&format!(" {} ", piece)[..]);
                    let value = if piece.color == Color::White { Colorize::green(value) } else { Colorize::red(value) };
                    if (i + j) % 2 == 1 {
                        print!("{}|", Colorize::on_black(value));
                    } else {
                        print!("{}|", Colorize::on_bright_black(value));
                    }
                },
                None => {
                    if (i + j) % 2 == 1 {
                        print!("{}|", Colorize::on_black("   "));
                    } else {
                        print!("{}|", Colorize::on_bright_black("   "));
                    }
                },
            }
        }
        print!( " |{}|", 8 - i);
        println!("");
        println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
    }
    println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
    println!("| | | A | B | C | D | E | F | G | H | | |");
    println!("+-+-+---+---+---+---+---+---+---+---+-+-+");
}

#[macro_export]
macro_rules! clear_view {
    () => {
      print!("\x1B[2J");
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

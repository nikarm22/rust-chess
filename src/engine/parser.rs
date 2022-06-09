use super::structs::piece::Piece;
use super::structs::castles_state::CastlesState;
use super::structs::enums::{Color, Position};
use super::structs::game_state::GameState;
use super::utils::std_pos_to_couple;

fn parse_position(position_string: String, state: &mut GameState) {
    let rows: Vec<String> = position_string
        .split("/")
        .map(str::to_string)
        .collect();

    for (row_index, row) in rows.iter().enumerate() {
        let mut current_column: u8 = 0;
        for letter in row.chars() {
            match letter.to_digit(10) {
                Some(value) => {
                    current_column += (value - 1) as u8;
                },
                None => {
                    let pos: Position = (row_index as u8, current_column);
                    let piece = Piece::from_fen_char(letter);
                    state.board.insert(pos, piece);
                }
            }
            current_column += 1;
        }
    }
}

pub fn parse(input: &mut String, state: &mut GameState) {
    let split: &mut Vec<String> = &mut input.split(" ").map(str::to_string).collect();

    if split.len() != 6 {
        panic!("Invalid FEN input provided!");
    }

    let position = split.remove(0);
    let whose_move = split.remove(0);
    let castles_state = split.remove(0);
    let en_pasant_position = split.remove(0);
    let half_moves = split.remove(0);
    let full_moves = split.remove(0);

    parse_position(position, state);
    state.whose_move = if whose_move == "w" { Color::White } else { Color::Black };
    state.castles = CastlesState::from_fen_str(castles_state);
    state.en_pasant_position = if en_pasant_position == "-" { None } else { Some(std_pos_to_couple(en_pasant_position)) };
    state.half_moves = half_moves.parse::<u16>().unwrap();
    state.full_moves = full_moves.parse::<u16>().unwrap();
}

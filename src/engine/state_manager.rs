use super::structs::game_state::GameState;
use super::structs::enums::{Color, PieceType, Position};
use super::move_generator::generate_valid_destinations;

pub fn execute_move(game_state: &mut GameState, from: Position, to: Position) -> Result<(), &'static str> {
    let piece = game_state.board.get(&from);

    let piece = match piece {
        Some(p) => p.clone(),
        None => { return Err("Piece is missing from source square!") },
    };
    let clone_piece = piece.clone();
    let valid_moves = generate_valid_destinations(game_state, piece.clone(), from);

    if !valid_moves.contains(&to) {
        return Err("Move is not valid!");
    }

    if game_state.whose_move != clone_piece.color {
        return Err("Wrong turn!");
    }

    // Executing move
    let piece = match game_state.board.remove(&from) {
        Some(p) => p,
        None => { return Err("Piece disappeared somehow!") },
    };
    let to_piece = game_state.board.remove(&to);
    game_state.board.insert(to, piece);

    // Increment move counter
    if clone_piece.color == Color::Black {
        game_state.full_moves += 1;
    }

    game_state.half_moves += 1;

    // If capture or pawn move, reseting 50 move rule
    if clone_piece.piece_type != PieceType::Pawn || to_piece.is_some() {
        game_state.half_moves = 0;
    }

    // Set up en pasant square if pawn moves 2 squares
    if clone_piece.piece_type == PieceType::Pawn && (to.0 - from.0).abs() == 2 {
        game_state.en_pasant_position = Some(((to.0 + from.0) / 2, to.1) as Position);
    } else {
        game_state.en_pasant_position = None;
    }

    // Update turn
    game_state.whose_move = if clone_piece.color == Color::White { Color::Black } else { Color::White };
    Ok(())
}

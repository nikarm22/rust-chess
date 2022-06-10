use crate::engine::structs::enums::{Color, Position, PieceType};
use crate::engine::structs::piece::Piece;
use crate::engine::structs::game_state::GameState;
use crate::engine::utils::is_in_bounds;
use std::vec::Vec;

pub fn generate_moves(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    match piece.piece_type {
        PieceType::Pawn => {generate_moves_pawn(game_state, piece, position)},
        PieceType::Bishop => {generate_moves_bishop(game_state, piece, position)},
        PieceType::Rook => {generate_moves_rook(game_state, piece, position)},
        PieceType::Queen => {generate_moves_queen(game_state, piece, position)},
        PieceType::King => {generate_moves_king(game_state, piece, position)},
        PieceType::Knight => {generate_moves_knight(game_state, piece, position)},
    }
}

#[derive(PartialEq, Debug)]
enum MoveStatus {
    Free,
    Capture,
    Blocked,
}

fn is_possible_move (game_state: &mut GameState, piece: Piece, next_position: Position) -> MoveStatus {
    if !is_in_bounds(next_position) {
        return MoveStatus::Blocked;
    }
    let collision = game_state.board.get(&next_position);

    match collision {
        Some(new_piece) => {
            if new_piece.color == piece.color {
                return MoveStatus::Blocked;
            } else {
                return MoveStatus::Capture;
            }
        },
        None => {
            return MoveStatus::Free;
        },
    }
}

fn generate_by_ranges (game_state: &mut GameState, piece: Piece, position: Position, x_increment: i8, y_increment: i8) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();
    for i in 1..8 {
        let next_position: Position = (position.0 + i*y_increment, position.1 + i*x_increment);
        let move_status = is_possible_move(game_state, piece.clone(), next_position);
        if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
            possible_positions.push(next_position);
        }

        if move_status == MoveStatus::Capture || move_status == MoveStatus::Blocked {
            break;
        }
    }

    return possible_positions;
}

pub fn generate_moves_pawn(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();
    let increment = if piece.color == Color::White { -1 } else { 1 };
    let is_double_move_allowed = (piece.color == Color::White && position.0 == 6) || (piece.color == Color::Black && position.0 == 1);

    let one_step = (position.0 + increment, position.1);
    let move_status = is_possible_move(game_state, piece.clone(), one_step);
    if move_status == MoveStatus::Free {
        possible_positions.push(one_step);

        if is_double_move_allowed {
            let two_step = (position.0 + 2*increment, position.1);
            let move_status = is_possible_move(game_state, piece.clone(), two_step);
            if move_status == MoveStatus::Free {
                possible_positions.push(two_step);
            }
        }
    }

    for i in [-1,1] {
        let next_position = (position.0 + increment, position.1 + i);
        let move_status = is_possible_move(game_state, piece.clone(), next_position);
        if move_status == MoveStatus::Capture {
            possible_positions.push(next_position);
        }
        match game_state.en_pasant_position {
            Some(_) => {
                if move_status == MoveStatus::Free {
                    possible_positions.push(next_position);
                }
            },
            None => {},
        }
    }

    possible_positions
}

pub fn generate_moves_rook(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();

    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, 1, 0));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, -1, 0));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, 0, 1));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, 0, -1));

    possible_positions
}

pub fn generate_moves_bishop(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();

    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, 1, 1));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, 1, -1));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, -1, 1));
    possible_positions.append(&mut generate_by_ranges(game_state, piece.clone(), position, -1, -1));

    possible_positions
}

pub fn generate_moves_queen(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();
    possible_positions.append(&mut generate_moves_bishop(game_state, piece.clone(), position));
    possible_positions.append(&mut generate_moves_rook(game_state, piece.clone(), position));

    possible_positions
}

pub fn generate_moves_king(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();
    for i in -1..1 {
        for j in -1..1 {
            if i == 0 && j == 0 {
                continue;
            }
            let next_position = (position.0 + i, position.1 + j);
            let move_status = is_possible_move(game_state, piece.clone(), next_position);
            if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
                possible_positions.push(next_position);
            }
        }
    }
    possible_positions
}

pub fn generate_moves_knight(game_state: &mut GameState, piece: Piece, position: Position) -> Vec<Position> {
    let mut possible_positions: Vec<Position> = Vec::new();
    for i in -2..2 {
        for j in -2..2 {
            if (i as i8).abs() + (j as i8).abs() != 3 {
                continue;
            }
            let next_position = (position.0 + i, position.1 + j);
            let move_status = is_possible_move(game_state, piece.clone(), next_position);
            if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
                possible_positions.push(next_position);
            }
        }
    }
    possible_positions
}

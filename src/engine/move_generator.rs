use crate::engine::structs::enums::{Color, Position, PieceType};
use crate::engine::structs::piece::Piece;
use crate::engine::structs::game_state::GameState;
use crate::engine::utils::is_in_bounds;
use std::collections::HashSet;

pub fn generate_moves(game_state: &mut GameState, piece: Piece, position: Position, only_attack: bool) -> HashSet<Position> {
    match piece.piece_type {
        PieceType::Pawn => {generate_moves_pawn(game_state, piece, position, only_attack)},
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

fn generate_by_ranges (game_state: &mut GameState, piece: Piece, position: Position, x_increment: i8, y_increment: i8) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    for i in 1..8 {
        let next_position: Position = (position.0 + i*y_increment, position.1 + i*x_increment);
        let move_status = is_possible_move(game_state, piece.clone(), next_position);
        if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
            possible_positions.insert(next_position);
        }

        if move_status == MoveStatus::Capture || move_status == MoveStatus::Blocked {
            break;
        }
    }

    return possible_positions;
}

pub fn generate_moves_pawn(game_state: &mut GameState, piece: Piece, position: Position, only_attack: bool) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    let increment = if piece.color == Color::White { -1 } else { 1 };
    let is_double_move_allowed = (piece.color == Color::White && position.0 == 6) || (piece.color == Color::Black && position.0 == 1);

    if !only_attack {
        let one_step = (position.0 + increment, position.1);
        let move_status = is_possible_move(game_state, piece.clone(), one_step);
        if move_status == MoveStatus::Free {
            possible_positions.insert(one_step);

            if is_double_move_allowed {
                let two_step = (position.0 + 2*increment, position.1);
                let move_status = is_possible_move(game_state, piece.clone(), two_step);
                if move_status == MoveStatus::Free {
                    possible_positions.insert(two_step);
                }
            }
        }
    }

    for i in [-1,1] {
        let next_position = (position.0 + increment, position.1 + i);
        let move_status = is_possible_move(game_state, piece.clone(), next_position);
        if move_status == MoveStatus::Capture {
            possible_positions.insert(next_position);
        }
        match game_state.en_pasant_position {
            Some(_) => {
                if move_status == MoveStatus::Free {
                    possible_positions.insert(next_position);
                }
            },
            None => {},
        }
    }

    possible_positions
}

pub fn generate_moves_rook(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();

    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, 1, 0));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, -1, 0));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, 0, 1));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, 0, -1));

    possible_positions
}

pub fn generate_moves_bishop(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();

    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, 1, 1));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, 1, -1));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, -1, 1));
    possible_positions.extend(&generate_by_ranges(game_state, piece.clone(), position, -1, -1));

    possible_positions
}

pub fn generate_moves_queen(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    possible_positions.extend(&generate_moves_bishop(game_state, piece.clone(), position));
    possible_positions.extend(&generate_moves_rook(game_state, piece.clone(), position));

    possible_positions
}

pub fn generate_moves_king(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    for i in -1..1 {
        for j in -1..1 {
            if i == 0 && j == 0 {
                continue;
            }
            let next_position = (position.0 + i, position.1 + j);
            let move_status = is_possible_move(game_state, piece.clone(), next_position);
            if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
                possible_positions.insert(next_position);
            }
        }
    }
    possible_positions
}

pub fn generate_moves_knight(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut possible_positions: HashSet<Position> = HashSet::new();
    for i in -2..2 {
        for j in -2..2 {
            if (i as i8).abs() + (j as i8).abs() != 3 {
                continue;
            }
            let next_position = (position.0 + i, position.1 + j);
            let move_status = is_possible_move(game_state, piece.clone(), next_position);
            if move_status == MoveStatus::Capture || move_status == MoveStatus::Free {
                possible_positions.insert(next_position);
            }
        }
    }
    possible_positions
}

fn find_king(game_state: &mut GameState, color: Color) -> Position {
    for (pos, piece) in game_state.board.iter() {
        if piece.color == color && piece.piece_type == PieceType::King {
            return *pos;
        }
    }
    (0, 0)
}

fn generate_attacked_fields(game_state: &mut GameState, color: Color) -> HashSet<Position> {
    let mut attacked_fields: HashSet<Position> = HashSet::new();
    for (pos, piece) in game_state.board.clone().iter() {
        if piece.color != color {
            attacked_fields.extend(&generate_moves(game_state, piece.clone(), pos.clone(), true));
        }
    }
    attacked_fields
}

fn fake_move(game_state: &mut GameState, from: Position, to: Position) -> GameState {
    let mut new_state = game_state.clone();
    let piece = new_state.board.remove(&from).unwrap();
    new_state.board.insert(to, piece);
    new_state
}

pub fn generate_valid_moves(game_state: &mut GameState, piece: Piece, position: Position) -> HashSet<Position> {
    let mut all_moves = generate_moves(game_state, piece.clone(), position, false);
    let mut illegal_moves: HashSet<Position> = HashSet::new();
    let color = piece.color;
    for possible_move in all_moves.iter() {
        let new_board = &mut fake_move(game_state, position, *possible_move);
        let attacked_fields = generate_attacked_fields(new_board, color);
        let king_pos = find_king(new_board, piece.color);
        if attacked_fields.contains(&king_pos) {
            illegal_moves.insert(*possible_move);
        }
    }

    all_moves.retain(|x| !illegal_moves.contains(x));
    all_moves
}

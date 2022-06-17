use crate::engine::move_generator::generate_attacked_fields;
use std::collections::HashSet;
use super::enums::{Color, Position, PieceType, GameResult};
use super::piece::Piece;
use super::board_map::BoardMap;
use super::castles_state::CastlesState;
use crate::engine::move_generator::generate_valid_destinations;

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: BoardMap,
    pub whose_move: Color,
    pub castles: CastlesState,
    pub en_pasant_position: Option<Position>,
    pub half_moves: u16,
    pub full_moves: u16,
    pub result: Option<GameResult>,
}

impl GameState {
    pub fn new () -> GameState {
        GameState {
            board: BoardMap::new(),
            castles: CastlesState::new(),
            en_pasant_position: None,
            full_moves: 1,
            half_moves: 0,
            whose_move: Color::White,
            result: None
        }
    }

    pub fn find_king(&self, color: Color) -> Option<Position> {
        self.board.iter().find(|(_, piece)| piece.color == color && piece.piece_type == PieceType::King).map(|(pos, _)| *pos)
    }

    pub fn fake_move(&self, from: Position, to: Position) -> GameState {
        let mut new_state = self.clone();
        if let Some(piece) = new_state.board.remove(&from) {
            new_state.board.insert(to, piece);
        }
        new_state
    }

    pub fn execute_move(&mut self, from: Position, to: Position, prom_piece: Option<Piece>) -> Result<(), &'static str> {
        if self.result.is_some() {
            return Err("Game is ended!");
        }

        let piece = self.board.get(&from).ok_or("Piece is missing from source square!")?;

        if piece.color != self.whose_move {
            return Err("Not your turn!");
        }

        let clone_piece = piece.clone();
        let valid_moves = generate_valid_destinations(self, piece.clone(), from);

        if !valid_moves.contains(&to) {
            return Err("Move is not valid!");
        }

        if self.whose_move != clone_piece.color {
            return Err("Wrong turn!");
        }

        // Executing move
        let old_piece = self.board.remove(&from);
        let piece = prom_piece.or(old_piece).ok_or("Piece disappeared somehow!")?;
        let to_piece = self.board.remove(&to);

        self.board.insert(to, piece);

        // Increment move counter
        if clone_piece.color == Color::Black {
            self.full_moves += 1;
        }

        self.half_moves += 1;

        // If capture or pawn move, reseting 50 move rule
        if clone_piece.piece_type != PieceType::Pawn || to_piece.is_some() {
            self.half_moves = 0;
        }

        // Set up en pasant square if pawn moves 2 squares
        if clone_piece.piece_type == PieceType::Pawn && (to.0 - from.0).abs() == 2 {
            self.en_pasant_position = Some(((to.0 + from.0) / 2, to.1) as Position);
        } else {
            self.en_pasant_position = None;
        }

        // Update turn
        self.whose_move = if clone_piece.color == Color::White { Color::Black } else { Color::White };

        self.check_game_ended();

        Ok(())
    }

    pub fn check_game_ended(&mut self) {
        let mut valid_moves: HashSet<Position> = HashSet::new();
        let current_color = self.whose_move;
        self.board
            .iter()
            .filter(|(_, piece)| piece.color == current_color)
            .for_each(|(pos, piece)| {
                valid_moves.extend(generate_valid_destinations(self, piece.clone(), *pos));
            });

        if valid_moves.len() == 0 {
            // TODO: Move to separate method is_under_check
            let attacked_fields = generate_attacked_fields(self, current_color);
            if let Some(king_pos) = self.find_king(current_color) {
                if attacked_fields.contains(&king_pos) {
                    self.result = if current_color == Color::White { Some(GameResult::BlackWin) } else { Some(GameResult::WhiteWin) };
                } else {
                    self.result = Some(GameResult::Stalement);
                }
            }
        }
    }
}

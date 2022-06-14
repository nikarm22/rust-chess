use super::enums::{Color, Position, PieceType};
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

    pub fn execute_move(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        let piece = self.board.get(&from);

        let piece = match piece {
            Some(p) => p.clone(),
            None => { return Err("Piece is missing from source square!") },
        };
        let clone_piece = piece.clone();
        let valid_moves = generate_valid_destinations(self, piece.clone(), from);

        if !valid_moves.contains(&to) {
            return Err("Move is not valid!");
        }

        if self.whose_move != clone_piece.color {
            return Err("Wrong turn!");
        }

        // Executing move
        let piece = match self.board.remove(&from) {
            Some(p) => p,
            None => { return Err("Piece disappeared somehow!") },
        };
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
        Ok(())
    }
}

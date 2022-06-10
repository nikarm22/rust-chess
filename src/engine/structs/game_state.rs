use super::enums::{Color, Position};
use super::board_map::BoardMap;
use super::castles_state::CastlesState;

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
}

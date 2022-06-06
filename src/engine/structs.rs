use std::collections::HashMap;

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
}

pub type BoardMap = HashMap<(i8,i8), Piece>;

#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug)]
pub struct GameState<'state_life> {
    pub board: &'state_life mut BoardMap,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
}

pub type Position = (i8, i8);
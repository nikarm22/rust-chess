#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
}

pub type Position = (i8, i8);
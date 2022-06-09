#[derive(Debug)]
#[derive(PartialEq)]
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

pub type Position = (u8, u8);
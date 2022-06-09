use super::enums::{PieceType, Color};
use std::fmt; 
// use std::char::from_u32;

#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type: piece_type, color: color }
    }

    pub fn from_fen_char(fen_char: char) -> Piece {
        let color = if fen_char.is_lowercase() { Color::Black } else { Color::White };
        let letter = fen_char.to_ascii_uppercase();
        let piece_type = match letter {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'R' => PieceType::Rook,
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            'P' => PieceType::Pawn,
            _ => PieceType::Pawn,
        };

        Piece::new(piece_type, color)
    }

    pub fn to_fen_char(&self) -> char {
        let letter = match self.piece_type {
            PieceType::King => { 'K' },
            PieceType::Queen => {'Q' },
            PieceType::Rook => { 'R' },
            PieceType::Bishop => { 'B' },
            PieceType::Knight => { 'N' },
            PieceType::Pawn => { 'P' },
        };

        let letter = if self.color == Color::Black { letter.to_ascii_lowercase() } else { letter };

        letter
    }
}

// impl fmt::Display for Piece {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut unicode_index: u32 = 0x2654;

//         match self.piece_type {
//             PieceType::King => { unicode_index += 0 },
//             PieceType::Queen => { unicode_index += 1 },
//             PieceType::Rook => { unicode_index += 2 },
//             PieceType::Bishop => { unicode_index += 3 },
//             PieceType::Knight => { unicode_index += 4 },
//             PieceType::Pawn => { unicode_index += 5 },
//         }

//         if self.color == Color::Black {
//             unicode_index += 6
//         }

//         match from_u32(unicode_index) {
//             Some(character) => {
//                 write!(f, "{}", character)
//             },
//             None => {write!(f, "{}", ' ')},
//         }
//     }
// }

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_fen_char())
    }
}
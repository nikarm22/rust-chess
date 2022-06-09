use super::enums::Position;
use super::piece::Piece;

use std::collections::HashMap;

pub type BoardMap = HashMap<Position, Piece>;

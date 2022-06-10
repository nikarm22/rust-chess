#[derive(Debug, Clone)]
pub struct CastlesState {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl CastlesState {
    pub fn new() -> CastlesState {
        CastlesState {
            white_king: false,
            white_queen: false,
            black_king: false,
            black_queen: false,
        }
    }

    pub fn from_fen_str(fen_str: String) -> CastlesState {
        let mut castles = CastlesState::new();

        for i in fen_str.chars() {
            match i {
                'K' => { castles.white_king = true },
                'Q' => { castles.white_queen = true },
                'k' => { castles.black_king = true },
                'q' => { castles.black_queen = true },
                _ => {},
            }
        }

        castles
    }
}

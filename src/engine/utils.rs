use super::structs::enums::Position;

pub fn std_pos_to_couple (std_pos: String) -> Result<Position, &'static str> {
  let bytes = std_pos.as_bytes();
  let row = bytes[0] as i8;
  let col = bytes[1] as i8;
  if row < 97 || row > 105 || col > 56 || col < 48 {
    return Err("Invalid Position!");
  }
  Ok((56 - bytes[1] as i8 , (bytes[0] as i8 - 97)))
}

pub fn is_in_bounds (position: Position) -> bool { position.0 >= 0 && position.0 < 8 && position.1 >= 0 && position.1 < 8 }

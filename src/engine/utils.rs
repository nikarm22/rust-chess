use super::structs::enums::Position;

pub fn std_pos_to_couple (std_pos: String) -> Position {
  let bytes = std_pos.as_bytes();
  if (bytes[0] as u8) < 97 || (bytes[1] as u8) > 56 {
    panic!("Invalid Position!");
  }
  (56 - bytes[1] as i8 , (bytes[0] as i8 - 97))
}

pub fn is_in_bounds (position: Position) -> bool { position.0 >= 0 && position.0 < 8 && position.1 >= 0 && position.1 < 8 }

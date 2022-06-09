use super::structs::enums::Position;

pub fn std_pos_to_couple (std_pos: String) -> Position {
  let bytes = std_pos.as_bytes();
  if (bytes[0] as u8) < 97 || (bytes[1] as u8) > 56 {
    panic!("Invalid Position!");
  }
  (56 - bytes[1] as u8 , (bytes[0] as u8 - 97))
}
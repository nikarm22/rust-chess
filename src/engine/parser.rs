pub fn parse(input: &mut String) {
  let split: Vec<String> = input.split(" ").map(str::to_string).collect();

  if split.len() != 6 {
      panic!("Invalid FEN input provided!");
  }

  let position = &split[0];
  println!("{}", position);


  let rows: Vec<String> = position
      .split("/")
      .map(str::to_string)
      .collect();

  for (row_index, row) in rows.iter().enumerate() {
      for (col_index, letter) in row.chars().enumerate() {
          println!("{} - {} - {}", row_index, col_index, letter);
      }
  }
}

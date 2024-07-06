use std::io;

pub fn get_input(prompt: &str) -> String {
  println!("{}", prompt);
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {},
    Err(_) => {}
  }

  return input.trim().to_string();
}

pub fn print_bitboard(bitboard: u64) {
  const LAST_BIT: u64 = 63;
  for rank in 0..8 {
      for file in (0..8).rev() {
          let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
          let char = if bitboard & mask != 0 { '1' } else { '0' };
          print!("{char} ");
      }
      println!();
  }
}
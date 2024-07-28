use std::{fmt::Error, io};

pub const NAME: &str = "Iridium";
pub const AUTHOR: &str = "TotalTaxAmount";

pub fn get_input(prompt: &str) -> String {
  println!("{}", prompt);
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {}
    Err(_) => {}
  }

  return input.trim().to_string();
}

pub fn pos_to_alph(pos: u8) -> Result<String, Error> {
  if pos > 63 {
    println!("{} is out of range", pos);
    return Err(Error);
  }

  let rank = pos / 8 + 1;
  let file = match pos % 8 {
    0 => "a",
    1 => "b",
    2 => "c",
    3 => "d",
    4 => "e",
    5 => "f",
    6 => "g",
    7 => "h",
    _ => {
      println!("{} is out of bounds!", pos);
      return Err(Error);
    }
  };

  Ok(format!("{}{}", file, rank))
}

pub fn alph_to_pos(alph: &str) -> Result<u8, Error> {
  let (f, r) = alph.split_at(1);
  let rank = match r.parse::<u8>() {
    Ok(r) => r - 1,
    Err(_) => {
      println!("Failed to convert to pos");
      return Err(Error);
    }
  };

  let file = match f {
    "a" => 0,
    "b" => 1,
    "c" => 2,
    "d" => 3,
    "e" => 4,
    "f" => 5,
    "g" => 6,
    "h" => 7,
    _ => {
      println!("{} is not a valid file!", f);
      return Err(Error);
    }
  };

  Ok(u8::from(file + rank * 8))
}

// https://stackoverflow.com/questions/2709430/count-number-of-bits-in-a-64-bit-long-big-integer
pub fn bitcount(mut i: u64) -> f32 {
  i = i - ((i >> 1) & 0x5555555555555555);

  i = (i & 0x3333333333333333) + ((i >> 2) & 0x3333333333333333);

  let sum = i + (i >> 4);
  let mask = 0xF0F0F0F0F0F0F0F;
  let factor = 0x101010101010101;

  let result = ((sum & mask).wrapping_mul(factor) >> 56) as f32;
  result
}

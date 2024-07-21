use std::{default, fmt::Display, io};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(usize)]
pub enum Sides {
    WHITE = 0,
    BLACK = 1,
}

impl Sides {
    pub fn from_usize(i: usize) -> Option<Sides>{
      match i {
          0 => Some(Self::WHITE),
          1 => Some(Self::BLACK),
          _ => None
      }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Pieces {
    PAWN = 0,
    BISHOP = 1,
    KNIGHT = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5,
}

impl Pieces {
    pub fn from_char(c: char) -> Option<(Sides, Pieces)> {
      let piece = match c {
        'p' | 'P' => Self::PAWN,
        'b' | 'B' => Self::BISHOP,
        'n' | 'N' => Self::KNIGHT,
        'r' | 'R' => Self::ROOK,
        'q' | 'Q' => Self::QUEEN,
        'k' | 'K' => Self::KING,
        _ => return None
      };

      Some((if c.is_uppercase() { Sides::WHITE } else { Sides::BLACK }, piece))
    }

    pub fn from_usize(i: usize) -> Option<Pieces> {
      match i {
         0 => Some(Self::PAWN),
         1 => Some(Self::BISHOP),
         2 => Some(Self::KNIGHT),
         3 => Some(Self::ROOK),
         4 => Some(Self::QUEEN),
         5 => Some(Self::KING),
         _ => None 
      }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
  // pub bb_sides: [BitBoard; 2],

  pub bb_pieces: [[BitBoard; 6]; 2],

  pub turn: Sides,

  pub white_can_oo: bool,

  pub black_can_oo: bool,

  pub white_can_ooo: bool,

  pub black_can_ooo: bool,

  pub en_passant_square: Option<u8>,

  pub half_moves: u64,

  pub full_moves: u64,

  pub score: u64,
}

impl Display for Board {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      println!("It is {:#?}'s turn", self.turn);
      println!("White castling rights -- oo: {}, ooo: {}", self.white_can_oo, self.white_can_ooo);
      println!("Black castling rights -- oo: {}, ooo: {}", self.black_can_oo, self.black_can_ooo);
      println!("Halfmoves: {}", self.half_moves);
      println!("Fullmoves: {}", self.full_moves);
      println!("En Passant target: {}", self.en_passant_square.unwrap());

      for (i, board) in self.bb_pieces.iter().enumerate() {
        println!("Side: {:?}", Sides::from_usize(i).unwrap());
        for (i, piece) in board.iter().enumerate() {
          println!("Piece: {:?}", Pieces::from_usize(i).unwrap());
          print_bitboard(*piece);
        }
      }
      Ok(())
    }
}

pub const NAME: &str = "Iridium";
pub const AUTHOR: &str = "TotalTaxAmount";

pub fn get_input(prompt: &str) -> String {
  println!("{}", prompt);
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => {},
    Err(_) => {}
  }

  return input.trim().to_string();
}

pub fn print_bitboard(bitboard: BitBoard) {
  const LAST_BIT: u64 = 63;
  for rank in 0..8 {
      for file in (0..8).rev() {
          let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
          let char = if bitboard.0 & mask != 0 { '1' } else { '0' };
          print!("{char} ");
      }
      println!();
  }
}
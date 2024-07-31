use core::fmt;
use std::{
  fmt::Display,
  ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
  vec,
};

use Iridium::pos_to_alph;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(usize)]
pub enum Sides {
  WHITE = 0,
  BLACK = 1,
}

impl Sides {
  pub fn from_usize(i: usize) -> Option<Self> {
    match i {
      0 => Some(Self::WHITE),
      1 => Some(Self::BLACK),
      _ => None,
    }
  }
}

impl Not for Sides {
  type Output = Sides;

  fn not(self) -> Self::Output {
    if self == Self::WHITE {
      Self::BLACK
    } else {
      Self::WHITE
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(usize)]
pub enum Pieces {
  #[default]
  PAWN = 0,
  BISHOP = 1,
  KNIGHT = 2,
  ROOK = 3,
  QUEEN = 4,
  KING = 5,
}

impl Pieces {
  pub fn from_char(c: char) -> Option<(Sides, Self)> {
    let piece = match c {
      'p' | 'P' => Self::PAWN,
      'b' | 'B' => Self::BISHOP,
      'n' | 'N' => Self::KNIGHT,
      'r' | 'R' => Self::ROOK,
      'q' | 'Q' => Self::QUEEN,
      'k' | 'K' => Self::KING,
      _ => return None,
    };

    Some((
      if c.is_uppercase() {
        Sides::WHITE
      } else {
        Sides::BLACK
      },
      piece,
    ))
  }

  pub fn from_usize(i: usize) -> Option<Self> {
    match i {
      0 => Some(Self::PAWN),
      1 => Some(Self::BISHOP),
      2 => Some(Self::KNIGHT),
      3 => Some(Self::ROOK),
      4 => Some(Self::QUEEN),
      5 => Some(Self::KING),
      _ => None,
    }
  }
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);
impl BitBoard {
  pub fn from_pos(pos: u8) -> Self {
    Self(1u64 << pos)
  }
}

impl BitAnd for BitBoard {
  type Output = BitBoard;

  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}

impl BitOr for BitBoard {
  type Output = BitBoard;

  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}

impl BitXor for BitBoard {
  type Output = BitBoard;

  fn bitxor(self, rhs: Self) -> Self::Output {
    Self(self.0 ^ rhs.0)
  }
}

impl Not for BitBoard {
  type Output = BitBoard;

  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}

impl Shl for BitBoard {
  type Output = BitBoard;

  fn shl(self, rhs: BitBoard) -> Self::Output {
    Self(self.0 << rhs.0)
  }
}

impl Shr for BitBoard {
  type Output = BitBoard;

  fn shr(self, rhs: Self) -> Self::Output {
    Self(self.0 >> rhs.0)
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
  // pub bb_sides: [BitBoard; 2],
  pub bb_pieces: [[BitBoard; 6]; 2],

  pub bb_sides: [BitBoard; 2],

  pub turn: Sides,

  pub white_can_oo: bool,

  pub black_can_oo: bool,

  pub white_can_ooo: bool,

  pub black_can_ooo: bool,

  pub en_passant_square: Option<u8>,

  pub half_moves: u64,

  pub full_moves: usize,

  pub score: f32,
}

impl Display for Board {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    println!("It is {:#?}'s turn", self.turn);
    println!(
      "White castling rights -- oo: {}, ooo: {}",
      self.white_can_oo, self.white_can_ooo
    );
    println!(
      "Black castling rights -- oo: {}, ooo: {}",
      self.black_can_oo, self.black_can_ooo
    );
    println!("Halfmoves: {}", self.half_moves);
    println!("Fullmoves: {}", self.full_moves);
    println!("En Passant target: {:?}", self.en_passant_square);

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

impl Default for Board {
  fn default() -> Self {
    Self {
      bb_pieces: [
        [
          BitBoard(65280), // Pawns
          BitBoard(36),    // Bishops
          BitBoard(66),    // Knights
          BitBoard(129),   // Rooks
          BitBoard(8),     // Queen
          BitBoard(16),    // King
        ],
        [
          BitBoard(71776119061217280),   // Pawns
          BitBoard(2594073385365405696), // Bishops
          BitBoard(4755801206503243776), // Knights
          BitBoard(9295429630892703744), // Rooks
          BitBoard(576460752303423488),  // Queen
          BitBoard(1152921504606846976), // King
        ],
      ],
      bb_sides: [BitBoard(65535), BitBoard(18446462598732840960)],
      turn: Sides::WHITE,
      white_can_oo: true,
      black_can_oo: true,
      white_can_ooo: true,
      black_can_ooo: true,
      en_passant_square: None,
      half_moves: 0,
      full_moves: 0,
      score: 0.0,
    }
  }
}

impl Board {
  pub fn apply_move(&mut self, m: Move) {
    //  let op_bb: BitBoard = if BitBoard::from_pos(m.start) & self.get_sides()[0] == BitBoard(0) { self.get_sides()[0] } else { self.get_sides()[1] };
    let side: Sides = if BitBoard::from_pos(m.start) & self.get_sides()[0] != BitBoard(0) {
      Sides::WHITE
    } else {
      Sides::BLACK
    };
    let mut piece: usize = 0;
    for (p, ps) in self.bb_pieces[side as usize].into_iter().enumerate() {
      if ps & BitBoard::from_pos(m.start) != BitBoard(0) {
        piece = p;
        break;
      }
    }

    // self.bb_pieces[side as usize][piece] =
    //   self.bb_pieces[side as usize][piece] ^ BitBoard::from_pos(m.start);
    self.bb_pieces[side as usize][piece] = self.bb_pieces[side as usize][piece]
      ^ (BitBoard::from_pos(m.dest) | BitBoard::from_pos(m.start));

    if m.capture != None {
      let op_side = if side == Sides::WHITE {
        Sides::BLACK
      } else {
        Sides::WHITE
      };
      self.bb_pieces[op_side as usize][m.capture.unwrap() as usize] =
        self.bb_pieces[op_side as usize][m.capture.unwrap() as usize] ^ BitBoard::from_pos(m.dest);
    }

    self.bb_sides = self.get_sides();
    self.full_moves += 1;
    //println!("{}", self.full_moves);
    // if self.full_moves % 2 == 0 {
    //   self.turn = Sides::WHITE;
    // } else {
    //   self.turn = Sides::BLACK;
    // }
    self.turn = !self.turn;
  }

  pub fn apply_moves(&mut self, moves: Vec<Move>) {
    for m in moves {
      self.apply_move(m);
    }
  }

  pub fn get_sides(&self) -> [BitBoard; 2] {
    let mut bb_sides: [BitBoard; 2] = [BitBoard(0); 2];
    for (s, pieces) in self.bb_pieces.into_iter().enumerate() {
      for p in pieces.into_iter() {
        bb_sides[s] = bb_sides[s] | p;
      }
    }
    bb_sides
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Move {
  pub start: u8,
  pub dest: u8,
  pub capture: Option<Pieces>,
}

#[derive(Clone, Debug)]
pub struct Line {
  moves: Vec<Move>,
}

impl Line {
  pub fn new() -> Self {
    Self { moves: vec![] }
  }

  pub fn add_move(&mut self, m: Move) {
    self.moves.push(m);
  }

  pub fn extend(&mut self, line: &Self) {
    self.moves.extend_from_slice(&line.moves);
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "currline ");
    for m in self.moves.clone() {
      write!(
        f,
        "{}{} ",
        pos_to_alph(m.start).unwrap(),
        pos_to_alph(m.dest).unwrap()
      );
    }
    write!(f, "")
  }
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

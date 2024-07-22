use core::fmt;
use std::{env::args, error, fmt::{write, Error}};

use Iridium::{Board, Sides, Pieces, BitBoard};

#[derive(Debug, Clone, Copy)]
pub struct FenError;
impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid FEN")
    }
}

impl error::Error for FenError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Failed to parse FEN"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

#[derive(PartialEq, Debug)]
pub struct UCITimer {
  time_msec: [i32; 2],
  inc_msec: [i32; 2],
  mtg: u32,
}

impl UCITimer {
    fn new() -> Self {
      UCITimer {
        time_msec: [0; 2],
        inc_msec: [0; 2],
        mtg: 0,
      }
    }

    fn blank(&self) -> bool {
      self.time_msec != [0; 2] &&
      self.inc_msec != [0; 2] &&
      self.mtg != 0
    }
}

impl fmt::Display for UCITimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "time: [ w: {}ms b: {}ms ] inc: [ w: {}ms b: {}ms ] moves to go: {}", 
        &self.time_msec[0],
        &self.time_msec[1],
        &self.inc_msec[0],
        &self.inc_msec[1],
        &self.mtg
      )
    }
}

#[derive(PartialEq, Debug)]
pub struct Constraints {
  time: Option<UCITimer>,
  depth: Option<u32>, 
  nodes: Option<u32>,
  mate: Option<u32>,
  movetime: Option<u32>,
  infinite: bool,
  ponder: bool
}

impl Constraints {
    pub fn new() -> Self {
      Constraints {
        time: Some(UCITimer::new()),
        depth: None,
        nodes: None,
        mate: None,
        movetime: None,
        infinite: false,
        ponder: false
      }
    }
}

pub struct Parsers;
impl Parsers {
    pub fn parse_time(time_args: &[&str]) -> Constraints{
      let mut token_id = 0;
      let mut constraints = Constraints::new();
      let mut time = UCITimer::new();
      while let Some(t) = time_args.get(token_id) {
        match *t {
          "infinite" => {
            constraints.infinite = true;
          },

          "ponder" => {
            constraints.ponder = true;
          },

          "wtime" => {
            if let Some(wtime) = time_args.get(token_id + 1) {
              if let Ok(t) = wtime.parse::<i32>() {
                time.time_msec[0] = t;
              }
              token_id += 1;
            }
          },

          "btime" => {
            if let Some(btime) = time_args.get(token_id + 1) {
              if let Ok(t) = btime.parse::<i32>() {
                time.time_msec[1] = t;
              }
              token_id += 1;
            }
          },

          "winc" => {
            if let Some(winc) = time_args.get(token_id + 1) {
              if let Ok(t) = winc.parse::<i32>() {
                time.inc_msec[0] = t;
              }
              token_id += 1;
            }
          },

          "binc" => {
            if let Some(binc) = time_args.get(token_id + 1) {
              if let Ok(t) = binc.parse::<i32>() {
                time.inc_msec[1] = t;
              }
              token_id += 1;
            }
          },

          "movestogo" => {
            if let Some(mtg) = time_args.get(token_id + 1) {
              if let Ok(t) = mtg.parse::<u32>() {
                time.mtg = t;
              }
              token_id += 1;
            }
          },

          "depth" => {
            if let Some(depth) = time_args.get(token_id + 1) {
              if let Ok(t) = depth.parse::<u32>() {
                constraints.depth = Some(t);
              }
              token_id += 1;
            }
          },

          "nodes" => {
            if let Some(nodes) = time_args.get(token_id + 1) {
              if let Ok(t) = nodes.parse::<u32>() {
                constraints.nodes = Some(t);
              }

              token_id += 1;
            }
          },

          "mate" => {
            if let Some(mate) = time_args.get(token_id + 1) {
              if let Ok(t) = mate.parse::<u32>() {
                constraints.mate = Some(t);
              }

              token_id += 1;
            }
          },

          "movetime" => {
            if let Some(movetime) = time_args.get(3) {
              if let Ok(t) = movetime.parse::<u32>() {
                constraints.movetime = Some(t);
              }
              token_id += 1;
            }
          }
          
          _ => {},
        }
        token_id += 1;
      }
      if !time.blank() {
        constraints.time = Some(time);
      }

      constraints
    }

    pub fn from_fen(fen: &[&str]) -> Result<Board, FenError> {
      // let parts: Vec<_> = fen.split(" ").collect();
      if fen.len() < 6 {
        println!("Invalid FEN");
        return Err(FenError);
      }
      
      let pos = Self::parse_position(fen[0]);
      let side_to_play = Self::parse_side_to_play(fen[1]);
      let castle_rights = Self::parse_castling(fen[2]);
      let en_passant = Self::parse_en_passant(fen[3]);
      let halfmoves = Self::parse_halfmoves(fen[4]);
      let fullmoves = Self::parse_fullmoves(fen[5]);

      Ok(Board {
        turn: side_to_play?,
        bb_pieces: pos?,
        white_can_oo: castle_rights?.0,
        black_can_oo: castle_rights?.1,
        white_can_ooo: castle_rights?.2,
        black_can_ooo: castle_rights?.3,
        en_passant_square: en_passant?,
        half_moves: halfmoves?,
        full_moves: fullmoves?,
        score: 0,
      })
  }

  fn parse_position(part: &str) -> Result<[[BitBoard; 6]; 2], FenError> {
    let ranks: Vec<_> = part.split("/").collect();
    let mut placement: [[BitBoard; 6]; 2] = [
      [BitBoard(0); 6],
      [BitBoard(0); 6]
    ];
  
    if ranks.len() != 8 {
      return Err(FenError)
    }

    for (rank, pieces) in ranks.iter().rev().enumerate() {
      let mut file = 0;

      for piece_char in pieces.chars() {
        match piece_char.to_digit(10) {
            Some(n) => file += n as usize,
            None => {
              let piece = Pieces::from_char(piece_char).ok_or(FenError);
              let bit_mask = rank * 8 + file;

              placement[piece.clone().unwrap().0 as usize][piece.unwrap().1 as usize].0 |= 1u64 << bit_mask;
              file += 1;
            }
          }
        }
      }
      Ok(placement)
  }

  fn parse_en_passant(part: &str) -> Result<Option<u8>, FenError> {
    if part == "-" {
      return Ok(None);
    }

    if part.len() != 2 {
      return Err(FenError);
    }

    let chars: Vec<_> = part.chars().collect();
    let (file, rank) = (chars[0], chars[1]);

    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 0,
    };


    let rank = match rank.to_digit(10) {
      Some(n) => n as u8 - 1,
      None => return Ok(None), 
    };

    Ok(Some(rank * 8 + file))
  }

  fn parse_side_to_play(part: &str) -> Result<Sides, FenError> {
    match part {
        "w" => Ok(Sides::WHITE),
        "b" => Ok(Sides::BLACK),
        _ => Err(FenError)
    }
  }

  fn parse_castling(part: &str) -> Result<(bool, bool, bool, bool), FenError> {
    Ok((
      part.contains("K"),
      part.contains("Q"),
      part.contains("k"),
      part.contains("q")
    ))
  }

  fn parse_fullmoves(part: &str) -> Result<u64, FenError> {
    match part.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(FenError),
    }
  }
  
  fn parse_halfmoves(part: &str) -> Result<u64, FenError> {
    match part.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(FenError),
    }
  }


}
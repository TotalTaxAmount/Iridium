use std::ascii::escape_default;
use std::fmt::Error;
use std::mem::MaybeUninit;

use crate::structs::{print_bitboard, BitBoard, Board, Move, Pieces};

use crate::lib::alph_to_pos;

use super::fen::Fen;

pub struct Position;
impl Position {
  pub fn parse_position(args: &[&str]) -> Result<Board, Error> {
    let mut board: Board = Board::default();
    let mut token_id = 0;
    while let Some(t) =  args.get(token_id) {
      match *t {
        "startpos" => {},
        "fen" => {
          if let Ok(b) = Fen::from_fen(&args[(token_id + 1)..(token_id + 6)]) {
            board = b;
          } else {
            println!("Error parsing fen");
            return Err(Error);
          }
        },
        "moves" => {
          let moves: Vec<Move> = vec![];
          for m in args[(token_id + 1)..].iter() {
            let (start_str, end_str) = m.split_at(2);
              let start = match alph_to_pos(start_str) {
                Ok(s) => s,
                Err(e) => {
                  println!("{}", e);
                  return Err(Error);
                }
              };

              let dest = match alph_to_pos(end_str) {
                Ok(e) => e,
                Err(e) => {
                  println!("{}", e);
                  return Err(Error);
                } 
              };

              let mut bmove: Move = Default::default();

              for (side, sides) in board.bb_pieces.into_iter().enumerate() {
                for (piece, pieces) in sides.into_iter().enumerate() {
                  if pieces & BitBoard::from_pos(start) != BitBoard(0) {
                    bmove.start = start;
                    bmove.dest = dest;
                    bmove.piece = match Pieces::from_usize(piece) {
                      Some(p) => p,
                      None => return Err(Error)
                    };
                  }
                }
              }

              println!("{:?}", bmove);
          }
    
        },
        _ => {}  
      }
      token_id += 1;
    }
  

    Ok(board)
  }
}
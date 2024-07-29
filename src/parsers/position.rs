use std::fmt::Error;

use crate::structs::{print_bitboard, BitBoard, Board, Move, Pieces, Sides};

use crate::lib::alph_to_pos;

use super::fen::Fen;

pub struct Position;
impl Position {
  pub fn parse_position(args: &[&str]) -> Result<Board, Error> {
    let mut board: Board = Board::default();
    let mut token_id = 0;
    while let Some(t) = args.get(token_id) {
      match *t {
        "startpos" => {}
        "fen" => {
          if let Ok(b) = Fen::from_fen(&args[(token_id + 1)..(token_id + 7)]) {
            board = b;
          } else {
            println!("Error parsing fen");
            return Err(Error);
          }
        }
        "moves" => {
          for (move_count, m) in args[(token_id + 1)..].iter().enumerate() {
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
              for pieces in sides.into_iter() {
                if pieces & BitBoard::from_pos(start) != BitBoard(0) {
                  bmove.start = start;
                  bmove.dest = dest;
                }

                for (piece, pieces) in board.bb_pieces[!Sides::from_usize(side).unwrap() as usize]
                  .into_iter()
                  .enumerate()
                {
                  if BitBoard::from_pos(dest) & pieces != BitBoard(0) {
                    bmove.capture = Pieces::from_usize(piece);
                  }
                }

                // if Sides::from_usize(side) != move_side
                //   && move_side != None
                // {
                //   for (piece, pieces) in board.bb_pieces[side].into_iter().enumerate() {
                //     if pieces & BitBoard::from_pos(dest) != BitBoard(0) {
                //       println!("Move {} ({}{}) is a capture", board.full_moves, start_str, end_str);
                //       bmove.capture = Pieces::from_usize(piece);
                //     }
                //   }
                // }
              }
            }

            board.apply_move(bmove);
            board.full_moves = move_count + 1;
            if (move_count + 1) % 2 == 0 {
              board.turn = Sides::WHITE;
            } else {
              board.turn = Sides::BLACK;
            }
          }
        }
        _ => {}
      }
      token_id += 1;
    }

    Ok(board)
  }
}

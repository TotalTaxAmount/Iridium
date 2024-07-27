use std::{fmt::DebugSet, vec};

use Iridium::pos_to_alph;

use crate::structs::{print_bitboard, BitBoard, Board, Move, Pieces, Sides};

pub struct MoveGen;
impl MoveGen {
  const ROW: i8 = 8;

  pub fn gen_moves(board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for (s, position) in board.bb_pieces.into_iter().enumerate() {
      if Sides::from_usize(s) != Some(side) {
        continue;
      }
      for (piece, bb) in position.into_iter().enumerate() {
        match Pieces::from_usize(piece) {
          Some(Pieces::PAWN) => moves.append(&mut Self::pawn_moves(bb, board, side)),
          Some(Pieces::BISHOP) => moves.append(&mut Self::bishop_moves(bb, board, side)),
          Some(Pieces::KNIGHT) => moves.append(&mut Self::knight_moves(bb)),
          Some(Pieces::ROOK) => moves.append(&mut Self::rock_moves(bb)),
          Some(Pieces::QUEEN) => moves.append(&mut Self::queen_moves(bb)),
          Some(Pieces::KING) => moves.append(&mut Self::king_moves(
            bb,
            if Sides::from_usize(s) == Some(Sides::WHITE) {
              board.white_can_oo
            } else {
              board.black_can_oo
            },
            if Sides::from_usize(s) == Some(Sides::WHITE) {
              board.white_can_ooo
            } else {
              board.black_can_ooo
            },
          )),

          None => continue,
        }
      }
    }
    moves
  }

  pub fn check_capture(p: u8, board: Board, side: Sides) -> Option<Pieces> {
    let mut capture: Option<Pieces> = None;

    if BitBoard::from_pos(p) & board.get_sides()[!side as usize] != BitBoard(0) {
      for (piece, pieces) in board.bb_pieces[!side as usize].into_iter().enumerate() {
        if pieces & BitBoard::from_pos(p) != BitBoard(0) {
          capture = Pieces::from_usize(piece);
        }
      }
    }
    capture
  }

  pub fn pawn_moves(pawns: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let empty_squares: BitBoard = !(board.get_sides()[0] | board.get_sides()[1]);

    let direction = if side == Sides::WHITE {
      Self::ROW
    } else {
      -Self::ROW
    };

    for s in 0..63 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & pawns != BitBoard(0) {
        let edge_dists: (u8, u8) = (
          //RIGHT, LEFT Shouldn't need top and bottom
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
        );

        let target_square = if (s as i8).overflowing_add(direction).1 {
          64 /* This becomes invalid by by the if statement below */
        } else {
          (s as i8).overflowing_add(direction).0
        } as u8;

        let double_square = if !(s as i8).overflowing_add(direction * 2).1
          && position_bb & Board::default().bb_pieces[side as usize][Pieces::PAWN as usize]
            != BitBoard(0)
        {
          (s as i8).overflowing_add(direction * 2).0
        } else {
          64
        } as u8;

        let capture_squares: (u8, u8) = (
          if (s as i8).overflowing_add(direction - 1).1 {
            64
          } else {
            (s as i8).overflowing_add(direction - 1).0
          } as u8,
          if (s as i8).overflowing_add(direction + 1).1 {
            64
          } else {
            (s as i8).overflowing_add(direction + 1).0
          } as u8,
        );

        if target_square <= 63 {
          let target_bb = BitBoard::from_pos(target_square);

          if target_bb & empty_squares != BitBoard(0) {
            moves.push(Move {
              start: s,
              dest: target_square,
              capture: None,
              mtype: "Single Pawn move".to_string(),
            });

            if double_square <= 63 {
              // Single moves has to be valid for the double move to be valid
              let target_bb = BitBoard::from_pos(double_square);

              if target_bb & empty_squares != BitBoard(0) {
                moves.push(Move {
                  start: s,
                  dest: double_square,
                  capture: None,
                  mtype: "Double Pawn Move".to_string(),
                });
              }
            }
          }
        }

        if capture_squares.0 <= 63 {
          if edge_dists.1 == 0 {
            continue;
          }

          let capture = Self::check_capture(capture_squares.0, board, side);

          if capture.is_some() {
            moves.push(Move {
              start: s,
              dest: capture_squares.0,
              capture,
              mtype: "Pawn Capture -1".to_string(),
            })
          }
        }

        if capture_squares.1 <= 63 {
          if edge_dists.0 == 0 {
            continue;
          }

          let capture = Self::check_capture(capture_squares.1, board, side);
          if capture.is_some() {
            moves.push(Move {
              start: s,
              dest: capture_squares.1,
              capture: capture,
              mtype: "Pawn Capture +1".to_string(),
            })
          }
        }
      }
    }
    moves
  }

  // Possible for this to use rays in future
  pub fn bishop_moves(bishops: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let empty_squares = !(board.get_sides()[0] | board.get_sides()[1]);

    for s in 0..63 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & bishops != BitBoard(0) {
        let edge_dists: (u8, u8) = (
          //RIGHT, LEFT Shouldn't need top and bottom
          8 - (s % 8),
          8 - (8 - (s % 8)) + 1,
        );

        // NE
        for i in 1..(edge_dists.0) {
          let dest = s + 9 * i;
          if dest > 63 || BitBoard::from_pos(dest) & board.get_sides()[side as usize] == BitBoard(0) {
            break;
          }

          let capture = Self::check_capture(dest, board, side);

          moves.push(Move {
            start: s,
            dest,
            capture,
            mtype: "Bishop NE".to_string(),
          });


          if capture.is_some() { break; } // Cant keep moving direction if there is a piece to capture
        }

        // NW
        for i in 1..edge_dists.1 {
          let dest = s + 7 * i;

          if dest > 63 || BitBoard::from_pos(dest) & empty_squares == BitBoard(0) {
            break;
          }

          let capture = Self::check_capture(dest, board, side);

          moves.push(Move {
            start: s,
            dest: s + 7 * i,
            capture,
            mtype: "Bishop NW".to_string(),
          });

          if capture.is_some() { break; }
        }

        // SW
        for i in 1..edge_dists.1 {
          if s.overflowing_sub(9 * i).1
            || BitBoard::from_pos(s - 9 * i) & empty_squares == BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - 9 * i, board, side);

          moves.push(Move {
            start: s,
            dest: s - 9 * i,
            capture,
            mtype: "Bishop SW".to_string(),
          });

          if capture.is_some() { break; }
        }

        // SE
        for i in 1..edge_dists.0 {
          if s.overflowing_sub(7 * i).1
            || BitBoard::from_pos(s - 7 * i) & empty_squares == BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - 7 * i, board, side);

          moves.push(Move {
            start: s,
            dest: s - 7 * i,
            capture,
            mtype: "Bishop SE".to_string(),
          });

          if capture.is_some() { break; }
        }
      }
    }

    println!("Bishop Moves: {:#?}", moves);
    moves
  }

  pub fn knight_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn rock_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn queen_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn king_moves(bb: BitBoard, castle_oo: bool, castle_ooo: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }
}

use std::{cmp::min, vec};

use Iridium::pos_to_alph;

use crate::structs::{BitBoard, Board, Move, Pieces, Sides};

pub struct MoveGen;
impl MoveGen {
  const ROW: i8 = 8;

  pub fn gen_moves(board: Board, side: Sides, legal_check: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for (piece, bb) in board.bb_pieces[side as usize].into_iter().enumerate() {
      match Pieces::from_usize(piece) {
        Some(Pieces::PAWN) => moves.append(&mut Self::pawn_moves(bb, board, side)),
        Some(Pieces::BISHOP) => moves.append(&mut Self::bishop_moves(bb, board, side)),
        Some(Pieces::KNIGHT) => moves.append(&mut Self::knight_moves(bb, board, side)),
        Some(Pieces::ROOK) => moves.append(&mut Self::rook_moves(bb, board, side)),
        Some(Pieces::QUEEN) => moves.append(&mut Self::queen_moves(bb, board, side)),
        Some(Pieces::KING) => moves.append(&mut Self::king_moves(bb, board, side)),

        None => continue,
      }
    }

    if legal_check {
      let mut legal_moves: Vec<Move> = vec![];
      for m in moves {
        let mut clone = board.clone();
        clone.apply_move(m);

        let opside_moves = Self::gen_moves(clone, !side, false);

        let mut is_legal = true;

        for op_move in opside_moves {
          if op_move.capture == Some(Pieces::KING) {
            is_legal = false;
            continue;
          }
        }

        if is_legal {
          legal_moves.push(m);
        }
      }
      moves = legal_moves;
    }

    moves
  }

  pub fn check_capture(p: u8, board: Board, side: Sides) -> Option<Pieces> {
    let mut capture: Option<Pieces> = None;

    if BitBoard::from_pos(p) & board.bb_sides[!side as usize] != BitBoard(0) {
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
    let empty_squares: BitBoard = !(board.bb_sides[0] | board.bb_sides[1]);

    let direction = if side == Sides::WHITE {
      Self::ROW
    } else {
      -Self::ROW
    };

    for s in 0..64 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & pawns != BitBoard(0) {
        let edge_dists: (u8, u8) = (
          //RIGHT, LEFT Shouldn't need top and bottom
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
        );

        let target_square = if (s as i8).overflowing_add(direction).1 {
          64 /* This becomes invalid by the if statement below */
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
            });

            if double_square <= 63 {
              // Single moves has to be valid for the double move to be valid
              let target_bb = BitBoard::from_pos(double_square);

              if target_bb & empty_squares != BitBoard(0) {
                moves.push(Move {
                  start: s,
                  dest: double_square,
                  capture: None,
                });
              }
            }
          }
        }

        if capture_squares.0 <= 63 {
          if edge_dists.1 != 0 {
            let capture = Self::check_capture(capture_squares.0, board, side);

            if capture.is_some() {
              //println!("{} -> {} cap 0 ed: {:?}", pos_to_alph(s).unwrap(), pos_to_alph(capture_squares.0).unwrap(), edge_dists);
              moves.push(Move {
                start: s,
                dest: capture_squares.0,
                capture,
              })
            }
          }
        }

        if capture_squares.1 <= 63 {
          if edge_dists.0 != 0 {
            let capture = Self::check_capture(capture_squares.1, board, side);
            if capture.is_some() {
              //println!("{} -> {} cap 1 ed: {:?}", pos_to_alph(s).unwrap(), pos_to_alph(capture_squares.1).unwrap(), edge_dists);
              moves.push(Move {
                start: s,
                dest: capture_squares.1,
                capture,
              })
            }
          }
        }
      }
    }
    moves
  }

  // Possible for this to use rays in future
  pub fn bishop_moves(bishops: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for s in 0..64 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & bishops != BitBoard(0) {
        let edge_dists: (u8, u8, u8, u8) = (
          //RIGHT, LEFT, TOP, BOTTOM
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
          8 - (s / 8) - 1,
          (s / 8),
        );

        // NE
        for i in 1..min(edge_dists.0, edge_dists.2) + 1 {
          let dest = s + 9 * i;
          if dest > 63 || BitBoard::from_pos(dest) & board.bb_sides[side as usize] != BitBoard(0) {
            break;
          }

          let capture = Self::check_capture(dest, board, side);

          moves.push(Move {
            start: s,
            dest,
            capture,
          });

          if capture.is_some() {
            break;
          } // Cant keep moving direction if there is a piece to capture
        }

        // NW
        for i in 1..min(edge_dists.1, edge_dists.2) + 1 {
          let dest = s + 7 * i;

          if dest > 63 || BitBoard::from_pos(dest) & board.bb_sides[side as usize] != BitBoard(0) {
            break;
          }

          let capture = Self::check_capture(dest, board, side);

          moves.push(Move {
            start: s,
            dest: s + 7 * i,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }

        // SW
        for i in 1..min(edge_dists.1, edge_dists.3) + 1 {
          if s.overflowing_sub(9 * i).1
            || BitBoard::from_pos(s - 9 * i) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - 9 * i, board, side);

          moves.push(Move {
            start: s,
            dest: s - 9 * i,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }

        // SE
        for i in 1..min(edge_dists.0, edge_dists.3) + 1 {
          if s.overflowing_sub(7 * i).1
            || BitBoard::from_pos(s - 7 * i) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - 7 * i, board, side);

          moves.push(Move {
            start: s,
            dest: s - 7 * i,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }
      }
    }
    moves
  }

  pub fn knight_moves(knights: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    const SHIFTS: [i8; 8] = [6, 10, 15, 17, -6, -10, -15, -17];

    for s in 0..64 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & knights != BitBoard(0) {
        let edge_dists: (u8, u8, u8, u8) = (
          //RIGHT, LEFT, TOP, BOTTOM
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
          8 - (s / 8) - 1,
          (s / 8),
        );

        for shift in SHIFTS {
          let dest = (s as i8).wrapping_add(shift);

          if dest > 63 || dest as u8 == s || dest < 0 {
            continue;
          }

          // EDGE DISTS
          // 0 = RIGHT
          // 1 = LEFT
          // 2 = TOP
          // 3 = BOTTOM

          // SHIFTS
          // 6 = UP 2LEFT
          // -6 = DOWN 2RIGHT
          // 10 = UP 2RIGHT
          // -10 = DOWN 2LEFT
          // 15 = 2UP LEFT
          // -15 = 2DOWN RIGHT
          // 17 = 2UP RIGHT
          // -17 = 2DOWN LEFT

          let is_valid_move = match shift {
            6 => edge_dists.1 >= 2 && edge_dists.2 >= 1,
            10 => edge_dists.0 >= 2 && edge_dists.2 >= 1,
            15 => edge_dists.1 >= 1 && edge_dists.2 >= 2,
            17 => edge_dists.0 >= 1 && edge_dists.2 >= 2,

            -6 => edge_dists.0 >= 2 && edge_dists.3 >= 1,
            -10 => edge_dists.1 >= 2 && edge_dists.3 >= 1,
            -15 => edge_dists.0 >= 1 && edge_dists.3 >= 2,
            -17 => edge_dists.1 >= 1 && edge_dists.3 >= 2,
            _ => false,
          };

          if is_valid_move {
            let dest_bb = BitBoard::from_pos(dest.try_into().unwrap());

            if dest_bb & board.bb_sides[side as usize] == BitBoard(0) {
              let capture = Self::check_capture(dest.try_into().unwrap(), board, side);

              moves.push(Move {
                start: s,
                dest: dest.try_into().unwrap(),
                capture,
              })
            }
          }
        }
      }
    }
    moves
  }

  pub fn rook_moves(rooks: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for s in 0..64 {
      let position_bb = BitBoard::from_pos(s);
      if position_bb & rooks != BitBoard(0) {
        let edge_dists: (u8, u8, u8, u8) = (
          //RIGHT, LEFT, TOP, BOTTOM
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
          8 - (s / 8) - 1,
          (s / 8),
        );

        for i in 1..(edge_dists.0 + 1) {
          if s + i > 63 || BitBoard::from_pos(s + i) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s + i, board, side);

          moves.push(Move {
            start: s,
            dest: s + i,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }

        for i in 1..(edge_dists.1 + 1) {
          if s.overflowing_sub(i).1
            || BitBoard::from_pos(s - i) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - i, board, side);

          moves.push(Move {
            start: s,
            dest: s - i,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }

        for i in 1..(edge_dists.2 + 1) {
          if s + i * 8 > 63
            || BitBoard::from_pos(s + i * 8) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s + i * 8, board, side);

          moves.push(Move {
            start: s,
            dest: s + i * 8,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }

        for i in 1..(edge_dists.3 + 1) {
          if s.overflowing_sub(i * 8).1
            || BitBoard::from_pos(s - i * 8) & board.bb_sides[side as usize] != BitBoard(0)
          {
            break;
          }

          let capture = Self::check_capture(s - i * 8, board, side);

          moves.push(Move {
            start: s,
            dest: s - i * 8,
            capture,
          });

          if capture.is_some() {
            break;
          }
        }
      }
    }
    moves
  }

  pub fn queen_moves(bb: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    moves.append(&mut Self::bishop_moves(bb, board, side));
    moves.append(&mut Self::rook_moves(bb, board, side));
    // println!("Queen Moves: {:#?} \n Len: {}", moves, moves.len());
    moves
  }

  pub fn king_moves(kings: BitBoard, board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let castle_rights: (bool, bool) = (
      if side == Sides::WHITE {
        board.white_can_oo
      } else {
        board.black_can_oo
      },
      if side == Sides::WHITE {
        board.white_can_ooo
      } else {
        board.black_can_ooo
      },
    );

    for s in 0..64 {
      let position_bb = BitBoard::from_pos(s);

      if position_bb & kings != BitBoard(0) {
        let edge_dists: (u8, u8, u8, u8) = (
          //RIGHT, LEFT, TOP, BOTTOM
          8 - (s % 8) - 1,
          8 - (8 - (s % 8)),
          8 - (s / 8) - 1,
          (s / 8),
        );

        if edge_dists.0 != 0 {
          if s + 1 < 63 && BitBoard::from_pos(s + 1) & board.bb_sides[side as usize] == BitBoard(0)
          {
            let capture = Self::check_capture(s + 1, board, side);

            moves.push(Move {
              start: s,
              dest: s + 1,
              capture,
            });
          }

          if edge_dists.2 != 0 {
            if s + 9 < 63
              && BitBoard::from_pos(s + 9) & board.bb_sides[side as usize] == BitBoard(0)
            {
              let capture = Self::check_capture(s + 9, board, side);

              moves.push(Move {
                start: s,
                dest: s + 9,
                capture,
              });
            }
          }

          if edge_dists.3 != 0 {
            if !s.overflowing_sub(7).1
              && BitBoard::from_pos(s - 7) & board.bb_sides[side as usize] == BitBoard(0)
            {
              let capture = Self::check_capture(s - 7, board, side);

              moves.push(Move {
                start: s,
                dest: s - 7,
                capture,
              });
            }
          }
        }

        if edge_dists.1 != 0 {
          if !s.overflowing_sub(1).1
            && BitBoard::from_pos(s - 1) & board.bb_sides[side as usize] == BitBoard(0)
          {
            let capture = Self::check_capture(s - 1, board, side);

            moves.push(Move {
              start: s,
              dest: s - 1,
              capture,
            });
          }

          if edge_dists.2 != 0 {
            if s + 7 < 63
              && BitBoard::from_pos(s + 7) & board.bb_sides[side as usize] == BitBoard(0)
            {
              let capture = Self::check_capture(s + 7, board, side);

              moves.push(Move {
                start: s,
                dest: s + 7,
                capture,
              });
            }
          }

          if edge_dists.3 != 0 {
            if !s.overflowing_sub(9).1
              && BitBoard::from_pos(s - 9) & board.bb_sides[side as usize] == BitBoard(0)
            {
              let capture = Self::check_capture(s - 9, board, side);

              moves.push(Move {
                start: s,
                dest: s - 9,
                capture,
              });
            }
          }
        }

        if edge_dists.2 != 0 {
          if s + 8 < 63 && BitBoard::from_pos(s + 8) & board.bb_sides[side as usize] == BitBoard(0)
          {
            let capture = Self::check_capture(s + 8, board, side);

            moves.push(Move {
              start: s,
              dest: s + 8,
              capture,
            });
          }
        }

        if edge_dists.3 != 0 {
          if !s.overflowing_sub(8).1
            && BitBoard::from_pos(s - 8) & board.bb_sides[side as usize] == BitBoard(0)
          {
            let capture = Self::check_capture(s - 8, board, side);

            moves.push(Move {
              start: s,
              dest: s - 8,
              capture,
            });
          }
        }
      }
    }
    moves
  }
}

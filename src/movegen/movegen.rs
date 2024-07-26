use std::{vec};

use crate::structs::{print_bitboard, BitBoard, Board, Move, Pieces, Sides};

pub struct MoveGen;
impl MoveGen {
  pub fn gen_moves(board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    for (s, position) in board.bb_pieces.into_iter().enumerate() {
      if Sides::from_usize(s) != Some(side) {
        continue;
      } 
      for (piece, bb) in position.into_iter().enumerate() {
        match Pieces::from_usize(piece) {
            Some(Pieces::PAWN) => moves.append(&mut Self::pawn_moves(board, side)),
            Some(Pieces::BISHOP) => moves.append(&mut Self::bishop_moves(bb)),
            Some(Pieces::KNIGHT) => moves.append(&mut Self::knight_moves(bb)),
            Some(Pieces::ROOK) => moves.append(&mut Self::rock_moves(bb)),
            Some(Pieces::QUEEN) => moves.append(&mut Self::queen_moves(bb)),
            Some(Pieces::KING) => moves.append(&mut Self::king_moves(bb, 
              if Sides::from_usize(s) == Some(Sides::WHITE) {
                board.white_can_oo
              } else {
                board.black_can_oo
              },
              if Sides::from_usize(s) == Some(Sides::WHITE) {
                board.white_can_ooo
              } else {
                board.black_can_ooo
              })),
            
            None => continue
        }
      }
    
    }

    moves
  }

  pub fn pawn_moves(board: Board, side: Sides) -> Vec<Move> {
    const ROW: i8 = 8;

    let mut moves: Vec<Move> = vec![];
    let pawns = board.bb_pieces[side as usize][Pieces::PAWN as usize];
    let empty_squares = !(board.get_sides()[0] | board.get_sides()[1]);

    let direction = if side == Sides::WHITE { ROW } else { -ROW };


    for s in 0..63 {
      let position_bb = BitBoard::from_pos(s);

      let target_square = if (s as i8).overflowing_add(direction).1 { continue; } else { (s as i8).overflowing_add(direction).0 } as u8;
      let double_square = if !(s as i8).overflowing_add(direction*2).1 && position_bb & Board::default().bb_pieces[side as usize][Pieces::PAWN as usize] != BitBoard(0) {
        (s as i8).overflowing_add(direction * 2).0 
      } else {
        64
      } as u8;

      if target_square <= 63 {
        let target_bb = BitBoard::from_pos(target_square);

        if position_bb & pawns != BitBoard(0) && target_bb & empty_squares != BitBoard(0) {
          moves.push(Move {
            start: s,
            dest: target_square,
            capture: None,
          });

          if double_square <= 63 {
            let target_bb = BitBoard::from_pos(double_square);
    
            if position_bb & pawns != BitBoard(0) && target_bb & empty_squares != BitBoard(0) {
              moves.push(Move {
                start: s,
                dest: double_square,
                capture: None,
              })
            }
          }
        }
      }

      // if side == Sides::WHITE {
      //   if board.bb_pieces[side as usize][Pieces::PAWN as usize] << BitBoard(8) & empty_squares & BitBoard::from_pos(s + 8) != BitBoard(0) && 
      //     BitBoard::from_pos(s) & board.bb_pieces[side as usize][Pieces::PAWN as usize] != BitBoard(0) {
      //       moves.push(Move { start: s, dest: s + 8, capture: None });
      //   }
      // } else {
      //   if board.bb_pieces[side as usize][Pieces::PAWN as usize] >> BitBoard(8) & empty_squares & BitBoard::from_pos(if s.overflowing_sub(8).1 {0} else {s - 8}) != BitBoard(0) && 
      //     BitBoard::from_pos(s) & board.bb_pieces[side as usize][Pieces::PAWN as usize] != BitBoard(0) {
      //       moves.push(Move { start: s, dest: s - 8, capture: None });
      //   }
      // }
    }
    moves
  }
  
  pub fn bishop_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

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
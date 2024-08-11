use std::arch::x86_64::_CMP_FALSE_OS;

use crate::{
  lib::bitcount,
  movegen::movegen::MoveGen,
  structs::{print_bitboard, Board, Line, Move, Pieces, Sides},
};

pub struct Engine {
  pub current_depth: u8
}
impl Engine {
  // Piece square tables
  const MG_PAWN_TABLE: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 98, 134, 61, 95, 68, 126, 34, -11, -6, 7, 26, 31, 65, 56, 25, -20, -14,
    13, 6, 21, 23, 12, 17, -23, -27, -2, -5, 12, 17, 6, 10, -25, -26, -4, -4, -10, 3, 3, 33, -12,
    -35, -1, -20, -23, -15, 24, 38, -22, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  const MG_KNIGHT_TABLE: [i16; 64] = [
    -167, -89, -34, -49, 61, -97, -15, -107, -73, -41, 72, 36, 23, 62, 7, -17, -47, 60, 37, 65, 84,
    129, 73, 44, -9, 17, 19, 53, 37, 69, 18, 22, -13, 4, 16, 13, 28, 19, 21, -8, -23, -9, 12, 10,
    19, 17, 25, -16, -29, -53, -12, -3, -1, 18, -14, -19, -105, -21, -58, -33, -17, -28, -19, -23,
  ];

  const MG_BISHOP_TABLE: [i16; 64] = [
    -29, 4, -82, -37, -25, -42, 7, -8, -26, 16, -18, -13, 30, 59, 18, -47, -16, 37, 43, 40, 35, 50,
    37, -2, -4, 5, 19, 50, 37, 37, 7, -2, -6, 13, 13, 26, 34, 12, 10, 4, 0, 15, 15, 15, 14, 27, 18,
    10, 4, 15, 16, 0, 7, 21, 33, 1, -33, -3, -14, -21, -13, -12, -39, -21,
  ];

  const MG_ROOK_TABLE: [i16; 64] = [
    32, 42, 32, 51, 63, 9, 31, 43, 27, 32, 58, 62, 80, 67, 26, 44, -5, 19, 26, 36, 17, 45, 61, 16,
    -24, -11, 7, 26, 24, 35, -8, -20, -36, -26, -12, -1, 9, -7, 6, -23, -45, -25, -16, -17, 3, 0,
    -5, -33, -44, -16, -20, -9, -1, 11, -6, -71, -19, -13, 1, 17, 16, 7, -37, -26,
  ];

  const MG_QUEEN_TABLE: [i16; 64] = [
    -28, 0, 29, 12, 59, 44, 43, 45, -24, -39, -5, 1, -16, 57, 28, 54, -13, -17, 7, 8, 29, 56, 47,
    57, -27, -27, -16, -16, -1, 17, -2, 1, -9, -26, -9, -10, -2, -4, 3, -3, -14, 2, -11, -2, -5, 2,
    14, 5, -35, -8, 11, 2, 8, 15, -3, 1, -1, -18, -9, 10, -15, -25, -31, -50,
  ];

  const MG_KING_TABLE: [i16; 64] = [
    -65, 23, 16, -15, -56, -34, 2, 13, 29, -1, -20, -7, -8, -4, -38, -29, -9, 24, 2, -16, -20, 6,
    22, -22, -17, -20, -12, -27, -30, -25, -14, -36, -49, -1, -27, -39, -46, -44, -33, -51, -14,
    -14, -22, -46, -44, -30, -15, -27, 1, 7, -8, -64, -43, -16, 9, 8, -15, 36, 12, -54, 8, -28, 24,
    14,
  ];

  pub fn new() -> Self {
    Self { current_depth: 0 }
  }

  pub fn evaluate(board: Board) -> f32 {
    let bb_pieces = board.bb_pieces;
    let mut score: f32 = 0.0;
    // Material
    score += 200.0
      * (bitcount(bb_pieces[0][Pieces::KING as usize].0)
        - bitcount(bb_pieces[1][Pieces::KING as usize].0))
      + 9.0
        * (bitcount(bb_pieces[0][Pieces::QUEEN as usize].0)
          - bitcount(bb_pieces[1][Pieces::QUEEN as usize].0))
      + 5.0
        * (bitcount(bb_pieces[0][Pieces::ROOK as usize].0)
          - bitcount(bb_pieces[1][Pieces::ROOK as usize].0))
      + 3.0
        * ((bitcount(bb_pieces[0][Pieces::KNIGHT as usize].0)
          - bitcount(bb_pieces[1][Pieces::KNIGHT as usize].0))
          + (bitcount(bb_pieces[0][Pieces::BISHOP as usize].0)
            - bitcount(bb_pieces[1][Pieces::BISHOP as usize].0)))
      + 1.0
        * (bitcount(bb_pieces[0][Pieces::PAWN as usize].0)
          - bitcount(bb_pieces[1][Pieces::PAWN as usize].0));

    // Mobility
    score += 0.1
      * (MoveGen::gen_moves(board, Sides::WHITE, true).len() as f32
        - MoveGen::gen_moves(board, Sides::BLACK, true).len() as f32);

    // for side in 0..2 {
    //   for piece in 0..6 {
    //     let mut bb = bb_pieces[side][piece];

    //     while bb.0 != 0 {
    //       let square = bb.0.trailing_zeros() as usize;
    //       bb.0 &= bb.0 - 1;

    //       let table_index = if side == 0 { square } else { 63 - square };
    //       let value = match Pieces::from_usize(piece) {
    //         Some(Pieces::PAWN) => Engine::MG_PAWN_TABLE[table_index],
    //         Some(Pieces::KNIGHT) => Engine::MG_KNIGHT_TABLE[table_index],
    //         Some(Pieces::BISHOP) => Engine::MG_BISHOP_TABLE[table_index],
    //         Some(Pieces::ROOK) => Engine::MG_ROOK_TABLE[table_index],
    //         Some(Pieces::QUEEN) => Engine::MG_QUEEN_TABLE[table_index],
    //         Some(Pieces::KING) => Engine::MG_KING_TABLE[table_index],
    //         None => 0,
    //       };

    //       score += if side == 0 { value } else { -value } as f32 * 0.004
    //     }
    //   }
    // }

    score
  }

  pub fn pvs(&mut self, board: Board, mut alpha: f32, beta: f32, depth: u8) -> f32 {
    if depth == 0 {
      return Engine::quiesce(board, alpha, beta);
    }
    
    self.current_depth = self.current_depth + 1;

    for m in MoveGen::gen_moves(board, board.turn, true) {
      let mut c_board = board.clone();
      c_board.apply_move(m);
      
      let mut score;
      if self.current_depth <= 1 {
        score = -self.pvs(c_board, -beta, -alpha, depth - 1);
      } else {
        score = -self.pvs(c_board, -alpha - 1.0, -alpha, depth - 1);

        if score > alpha && beta - alpha > 1.0 {
          score = -self.pvs(c_board, -beta, -alpha, depth - 1);
        } 
      }

      if score >= beta {
        return beta;
      }
      if score > alpha {
        alpha = score;
      }
    }

    return alpha;
  }

  fn quiesce(board: Board, mut alpha: f32, beta: f32) -> f32 {
    let eval = Self::evaluate(board);
    if eval >= beta {
      return beta;
    }

    if alpha < eval {
      alpha = eval;
    }

    for m in MoveGen::gen_moves(board, board.turn, true) {
      if m.capture != None {
        let mut c_board = board.clone();
        c_board.apply_move(m);

        let score = -Self::quiesce(c_board, -beta, -alpha);

        if score >= beta {
          return beta;
        }
        if score > alpha {
          alpha = score;
        }
      }
    }

    return alpha;
  }
}

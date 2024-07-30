use std::{cmp::max, env::var, f32::INFINITY};

use crate::{
  lib::bitcount,
  movegen::movegen::MoveGen,
  structs::{print_bitboard, Board, Pieces, Sides},
};

pub struct Engine;
impl Engine {
  pub fn evaluate(board: Board) -> f32 {
    let bb_pieces = board.bb_pieces;
    let mut score: f32 = 0.0;
    score = 200.0
      * (bitcount(bb_pieces[0][Pieces::KING as usize].0)
        - bitcount(bb_pieces[1][Pieces::KING as usize].0))
      + 9.0
        * (bitcount(bb_pieces[0][Pieces::QUEEN as usize].0)
          - bitcount(bb_pieces[1][Pieces::QUEEN as usize].0))
      + 5.0
        * (bitcount(bb_pieces[0][Pieces::ROOK as usize].0)
          - bitcount(bb_pieces[1][Pieces::ROOK as usize].0))
      + 3.0
        * (bitcount(bb_pieces[0][Pieces::KNIGHT as usize].0)
          - bitcount(bb_pieces[1][Pieces::KNIGHT as usize].0)
          + bitcount(bb_pieces[0][Pieces::BISHOP as usize].0)
          - bitcount(bb_pieces[1][Pieces::BISHOP as usize].0))
      + 1.0
        * (bitcount(bb_pieces[0][Pieces::PAWN as usize].0)
          - bitcount(bb_pieces[1][Pieces::PAWN as usize].0))
      + 0.1
        * (MoveGen::gen_moves(board, Sides::WHITE, true).len() as f32
          - MoveGen::gen_moves(board, Sides::BLACK, true).len() as f32);

    score
  }

  // pub fn bestmove(board: Board, side: Sides, depth: u8) -> Option<Move> {
  //   let moves = MoveGen::gen_moves(board, side, true);

  //   let mut best_move = None;
  //   let mut best_eval = if side == Sides::WHITE {
  //     -INFINITY
  //   } else {
  //     INFINITY
  //   };

  //   for m in moves {
  //     let mut clone_board = board.clone();
  //     clone_board.apply_move(m);
  //     let eval = Self::alpha_beta_max(clone_board, side, -INFINITY, INFINITY, depth);
  //     println!("eval: {}", eval);
  //     if (side == Sides::WHITE && eval > best_eval) || (side == Sides::BLACK && eval < best_eval) {
  //       best_eval = eval;
  //       best_move = Some(m);
  //     }
  //   }

  //   println!("Best Move: {:?}, Best Evaluation: {}", best_move, best_eval);
  //   best_move
  // }

  // pub fn alpha_beta_max(board: Board, mut alpha: f32, beta: f32, depth: u8) -> f32 {
  //   if depth == 0 {
  //     return Self::evaluate(board);
  //   }

  //   let mut best_value = -INFINITY;
  //   for m in MoveGen::gen_moves(board, board.turn, true) {
  //     let mut clone_board = board.clone();
  //     clone_board.apply_move(m);
  //     let score = Self::alpha_beta_min(clone_board, alpha, beta, depth - 1);

  //     if score > best_value {
  //       best_value = score;
  //       if score > alpha {
  //         alpha = score;
  //       }
  //     }
  //     if score >= beta {
  //       return score;
  //     }
  //   }

  //   best_value
  // }

  // pub fn alpha_beta_min(board: Board, alpha: f32, mut beta: f32, depth: u8) -> f32 {
  //   if depth == 0 {
  //     return -Self::evaluate(board);
  //   }

  //   let mut best_value = INFINITY;
  //   for m in MoveGen::gen_moves(board, board.turn, true) {
  //     let mut clone_board = board.clone();
  //     clone_board.apply_move(m);
  //     let score = Self::alpha_beta_max(clone_board, alpha, beta, depth - 1);

  //     if score < best_value {
  //       best_value = score;
  //       if score < beta {
  //         beta = score;
  //       }
  //     }
  //     if score <= alpha {
  //       return score;
  //     }
  //   }

  //   best_value
  // }

  pub fn nega_scout(board: Board, depth: u8, mut alpha: f32, beta: f32) -> f32 {
    if depth == 0 {
      // let color = if board.turn == Sides::WHITE { 1.0 } else { -1.0 };
      return Self::evaluate(board);
    }

    let mut best_value = -INFINITY;
    let mut n = beta;

    for m in MoveGen::gen_moves(board, board.turn, true) {
      let mut clone_board = board.clone();
      clone_board.apply_move(m);
      // println!();
      // print_bitboard(clone_board.bb_sides[0] | clone_board.bb_sides[1]);

      let score = -Self::nega_scout(clone_board, depth - 1, -n, -alpha);

      if score > best_value {
        if alpha < score && score < beta {
          best_value = score.max(best_value);
        } else {
          best_value = -Self::nega_scout(clone_board, depth - 1, -beta, -score);
        }
      }
      if score > alpha {
        alpha = score;
      };

      if alpha >= beta {
        return alpha;
      };

      n = alpha + 1.0;
    }

    best_value
  }
}

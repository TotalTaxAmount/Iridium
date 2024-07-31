use crate::{
  lib::bitcount,
  movegen::movegen::MoveGen,
  structs::{print_bitboard, Board, Line, Pieces, Sides},
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

  pub fn pvs(board: Board, alpha: f32, beta: f32, depth: u8) -> f32 {
    fn pvs_internal(board: Board, mut alpha: f32, beta: f32, depth: u8, f: bool) -> f32 {
      if depth == 0 {
        return Engine::evaluate(board);
      }

      for m in MoveGen::gen_moves(board, board.turn, true) {
        let mut clone_board = board.clone();
        clone_board.apply_move(m.clone());

        let mut score;

        if f {
          score = pvs_internal(clone_board, -beta, -alpha, depth - 1, false);
        } else {
          score = -Engine::zw_search(clone_board, -alpha, depth - 1);
          if score > alpha {
            score = -pvs_internal(clone_board, -beta, -alpha, depth - 1, false);
          }
        }

        if score >= beta {
          return beta;
        }

        if score > alpha {
          alpha = score;
        }
      }

      println!("PVS: Depth: {} Alpha: {} Beta: {}", depth, alpha, beta);
      alpha
    }

    pvs_internal(board, alpha, beta, depth, true)
  }

  pub fn zw_search(board: Board, beta: f32, depth: u8) -> f32 {
    if depth == 0 {
      return Self::evaluate(board);
    }

    for m in MoveGen::gen_moves(board, board.turn, true) {
      let mut clone_board = board.clone();
      clone_board.apply_move(m.clone());

      let score = -Self::zw_search(clone_board, 1.0 - beta, depth - 1);

      if score >= beta {
        return beta;
      }
    }
    return beta - 1.0;
  }
}

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

  pub fn pvs(board: Board, depth: u8, alpha: f32, beta: f32, side: Sides) -> (f32, Line) {
    fn pvs_internal(
      board: &Board, depth: u8, mut alpha: f32, beta: f32, side: Sides, f: bool,
    ) -> (f32, Line) {
      if depth == 0 {
        return (
          self::Engine::evaluate(*board) * if side == Sides::WHITE { 1.0 } else { -1.0 },
          Line::new(),
        );
      };

      let mut best_line = Line::new();

      for m in MoveGen::gen_moves(*board, board.turn, true) {
        let mut clone = board.clone();
        clone.apply_move(m);

        let mut score = 0.0;
        let mut curr_line = Line::new();

        if f {
          (score, curr_line) = pvs_internal(&clone, depth - 1, -beta, -alpha, !side, false);
          score = -score;
        } else {
          (score, curr_line) = pvs_internal(&clone, depth - 1, -alpha - 1.0, -alpha, !side, false);
          score = -score;
        }

        if alpha < score && score < beta {
          (score, curr_line) = pvs_internal(&clone, depth - 1, -beta, -alpha, !side, false);
          score = -score;
        }

        if score > alpha {
          alpha = score;
          best_line = curr_line.clone();
          best_line.add_move(m.clone());
        }

        if alpha >= beta {
          break;
        }
      }

      (alpha, best_line)
    }

    pvs_internal(&board, depth, alpha, beta, side, true)
  }
}

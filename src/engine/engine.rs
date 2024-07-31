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

    score * if board.turn == Sides::WHITE { 1.0 } else { -1.0 }
  }

  pub fn pvs(board: Board, mut alpha: f32, beta: f32, depth: u8) -> f32 {
    if depth == 0 {
      return Engine::evaluate(board);
    }

    for m in MoveGen::gen_moves(board, board.turn, true) {
      let mut c_board = board.clone();
      c_board.apply_move(m);
      println!("{:?}", c_board.turn);

      let score = -Engine::pvs(c_board, -beta, -alpha, depth - 1);

      if score > alpha {
        alpha = score;
      }
    }

    return alpha;
  }
}

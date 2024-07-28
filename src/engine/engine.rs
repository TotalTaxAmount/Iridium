use rand::{seq::IteratorRandom, thread_rng};
use Iridium::bitcount;

use crate::{
  movegen::movegen::MoveGen,
  structs::{Board, Move, Pieces, Sides},
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

  pub fn bestmove(board: Board, side: Sides, depth: u8) -> Option<Move> {
  }
}

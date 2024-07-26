use rand::{seq::IteratorRandom, thread_rng};

use crate::structs::{Board, Move};

pub struct Engine;
impl Engine {
  pub fn evaluate(board: Board) -> u8 {
    0
  }

  pub fn pick_move(moves: Vec<Move>) -> Option<Move> {
    moves.into_iter().choose(&mut thread_rng())
  }
}

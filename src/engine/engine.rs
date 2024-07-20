use Iridium::Board;

use crate::movegen::structs::Move;

pub struct Engine;
impl Engine {
  pub fn evaluate(board: Board) -> u8 {
    0
  }

  pub fn pick_move(moves: Vec<Move>) -> Move {
    moves[0]
  }
}
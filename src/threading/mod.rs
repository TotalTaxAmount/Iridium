use std::{
  f32::{consts::E, INFINITY},
  thread::{Builder, JoinHandle},
  vec,
};

use Iridium::pos_to_alph;

use crate::{
  engine::{self, engine::Engine},
  movegen::movegen::MoveGen,
  structs::{Board, Line, Move, Sides},
};

pub struct ThreadPool {
  pub threads: Vec<JoinHandle<(f32, Option<Move>, Line)>>,
  pub limit: u8,
}

impl ThreadPool {
  pub fn new(thread_limit: u8) -> Self {
    Self {
      threads: vec![],
      limit: thread_limit,
    }
  }

  pub fn search(&mut self, board: Board, side: Sides, depth: u8) -> Option<(Move, Line)> {
    let moves: Vec<Move> = MoveGen::gen_moves(board.clone(), side, true);
    let num_moves = moves.len().try_into().unwrap();
    let mut best_move: Option<(Move, Line)> = None;
    let mut best_eval = if side == Sides::WHITE {
      -INFINITY
    } else {
      INFINITY
    };

    let thread_limit = self.limit.min(num_moves);

    let chuck_size = num_moves.div_ceil(thread_limit);

    for (i, chunk) in moves
      .chunks(chuck_size.into())
      .map(|x| x.to_vec())
      .enumerate()
    {

      let builder = Builder::new().name(format!("Eval thread builder {}", i).into());

      let handle = builder.spawn(move || {
        let mut best_eval = if side == Sides::WHITE {
          -INFINITY
        } else {
          INFINITY
        };

        // let mut best_move = chunk[0];
        // let mut best_line = Line::new();

        // let mut engine = Engine::new();
        let mut engine: Engine = Engine::new();

        let res = engine.alpha_beta_max(board.clone(), chunk, -INFINITY, INFINITY, depth - 1, Line::new());

        

        (res.0, res.1.clone().get(0) , res.1)
      });

      match handle {
        Ok(h) => self.threads.push(h),
        Err(_) => {}
      }
    }

    for handle in self.threads.drain(..) {
      if let Ok((eval, m, line)) = handle.join() {
        if (side == Sides::WHITE && eval > best_eval) || (side == Sides::BLACK && eval < best_eval)
        // if eval > best_eval
        {
          best_eval = eval;
          best_move = match m {
              Some(m) => Some((m, line)),
              None => None
          }
        }
      }
    }

    println!("Best eval: {}, Best Moves {:?}", best_eval, best_move);
    best_move
  }
}

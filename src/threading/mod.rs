use std::{
  f32::INFINITY,
  thread::{Builder, JoinHandle},
  vec,
};

use Iridium::pos_to_alph;

use crate::{
  engine::engine::Engine,
  movegen::movegen::MoveGen,
  structs::{Board, Move, Sides},
};

pub struct ThreadPool {
  pub threads: Vec<JoinHandle<(f32, Move)>>,

  pub limit: u8,
}

impl ThreadPool {
  pub fn new(thread_limit: u8) -> Self {
    Self {
      threads: vec![],
      limit: thread_limit,
    }
  }

  pub fn search(&mut self, board: Board, side: Sides, depth: u8) -> Option<Move> {
    let moves: Vec<Move> = MoveGen::gen_moves(board.clone(), side, true);
    let num_moves = moves.len() as u8;
    let mut best_move: Option<Move> = None;

    let thread_limit = self.limit.min(num_moves);

    let chuck_size = (moves.len() as f32 / thread_limit as f32).ceil() as usize;

    for (i, chunk) in moves.chunks(chuck_size).map(|x| x.to_vec()).enumerate() {
      let clone_board = board.clone();

      let chunk_moves = chunk.to_vec();

      let builder = Builder::new().name(format!("Eval thread builder {}", i).into());

      let handle = builder.spawn(move || {
        let mut best_eval = -INFINITY;

        let mut best_move = chunk[0];

        for m in chunk_moves {
          let mut clone_board = clone_board.clone();
          clone_board.apply_move(m.clone());
          // let eval = if side == Sides::WHITE {
          //   Engine::alpha_beta_max(clone_board, side, -INFINITY, INFINITY, depth - 1)
          // } else {
          //   Engine::alpha_beta_min(clone_board, side, -INFINITY, INFINITY, depth - 1)
          // };
          let eval = -Engine::alpha_beta(clone_board, -INFINITY, INFINITY, depth - 1);

          println!(
            "{} - {:?}{:?} - {:?}",
            eval,
            pos_to_alph(m.start).unwrap(),
            pos_to_alph(m.dest).unwrap(),
            m.capture
          );

          if eval > best_eval {
            best_eval = eval;
            best_move = m;
          }
        }
        (best_eval, best_move)
      });

      match handle {
        Ok(h) => self.threads.push(h),
        Err(_) => {}
      }
    }

    let mut best_eval = -INFINITY;

    for handle in self.threads.drain(..) {
      if let Ok((eval, m)) = handle.join() {
        if eval > best_eval {
          best_eval = eval;
          best_move = Some(m);
        }
      }
    }

    println!("Best eval: {}, Best Moves {:?}", best_eval, best_move);
    best_move
  }
}

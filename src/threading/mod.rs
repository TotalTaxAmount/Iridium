use std::{
  f32::INFINITY,
  thread::{Builder, JoinHandle},
  vec,
};

use Iridium::pos_to_alph;

use crate::{
  engine::engine::Engine,
  movegen::movegen::MoveGen,
  structs::{Board, Line, Move, Sides},
};

pub struct ThreadPool {
  pub threads: Vec<JoinHandle<(f32, Move, Line)>>,

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
    let num_moves = moves.len() as u8;
    let mut best_move: Option<(Move, Line)> = None;
    // let mut best_eval = if side == Sides::WHITE {
    //   -INFINITY
    // } else {
    //   INFINITY
    // };
    let mut best_eval = -INFINITY;

    let thread_limit = self.limit.min(num_moves);

    let chuck_size = (moves.len() as f32 / thread_limit as f32).ceil() as usize;

    println!("{:?}", side);

    for (i, chunk) in moves.chunks(chuck_size).map(|x| x.to_vec()).enumerate() {
      let clone_board = board.clone();

      let builder = Builder::new().name(format!("Eval thread builder {}", i).into());

      let handle = builder.spawn(move || {
        // let mut best_eval = if side == Sides::WHITE {
        //   -INFINITY
        // } else {
        //   INFINITY
        // };

        let mut best_eval = -INFINITY;

        let mut best_move = chunk[0];
        let mut best_line = Line::new();

        for m in chunk {
          let mut clone_board = clone_board.clone();
          clone_board.apply_move(m);

          let mut engine = Engine::new();

          let pvs = engine.pvs(clone_board, -INFINITY, INFINITY, depth - 1, Line::new());
          let eval = -pvs.0;
          let line = pvs.1;

          println!(
            "{} - {}{} - {:?} :: calls {}, line: {}",
            eval,
            pos_to_alph(m.start).unwrap(),
            pos_to_alph(m.dest).unwrap(),
            m.capture,
            engine.current_depth,
            line
          );

          // if (side == Sides::WHITE && eval > best_eval)
          //   || (side == Sides::BLACK && eval < best_eval)
          // {
          if eval > best_eval {
            best_eval = eval;
            best_move = m;
          } else {
            // println!("{:?} -- {} | {} {}", side, eval, best_eval, eval < best_eval);
          }
        }
        (best_eval, best_move, Line::new())
      });

      match handle {
        Ok(h) => self.threads.push(h),
        Err(_) => {}
      }
    }

    for handle in self.threads.drain(..) {
      if let Ok((eval, m, line)) = handle.join() {
        // if (side == Sides::WHITE && eval > best_eval) || (side == Sides::BLACK && eval < best_eval)
        if eval > best_eval
        {
          best_eval = eval;
          best_move = Some((m, line));
        }
      }
    }

    println!("Best eval: {}, Best Moves {:?}", best_eval, best_move);
    best_move
  }
}

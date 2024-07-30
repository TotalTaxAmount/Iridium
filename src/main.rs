use engine::engine::Engine;
use movegen::movegen::MoveGen;
use parsers::{
  position::Position,
  time::{Constraints, Time},
};
use std::process::exit;
use structs::{print_bitboard, Board};
use threading::ThreadPool;
use Iridium::pos_to_alph;

mod engine;
mod lib;
mod movegen;
mod parsers;
mod structs;
mod threading;

fn ucimode() {
  // Identification
  println!("id name {}", lib::NAME);
  println!("id author {}", lib::AUTHOR);
  // Options

  // Ready
  println!("uciok");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut board: Board = Board::default();
  let mut constraints: Constraints;

  loop {
    let input = lib::get_input("");
    let args: Vec<&str> = input.split(" ").collect();
    let command: &str = args.first().unwrap_or(&"");
    match command {
      "uci" => ucimode(),
      "isready" => {
        println!("readyok")
      }
      "setoption" => {}
      "register" => {}
      "ucinewgame" => {}
      "position" => match Position::parse_position(&args) {
        Ok(b) => board = b,
        Err(e) => {
          println!("{}", e);
          continue;
        }
      },
      "go" => {
        constraints = Time::parse_time(&args);
        let moves = MoveGen::gen_moves(board, board.turn, true);
        let mut thread_pool = ThreadPool::new(30);

        println!("possible moves {}", moves.len());
        let best_move = match thread_pool.search(board, board.turn, 3) {
          Some(m) => m,
          None => continue,
        };
        println!("{:?} {}", board.turn, board.full_moves);
        board.apply_move(best_move);
        println!("info score cp {}", Engine::evaluate(board));
        println!(
          "bestmove {}{}",
          pos_to_alph(best_move.start)?,
          pos_to_alph(best_move.dest)?
        );
      }
      "stop" => {}
      "ponder" => {}
      "ponderhint" => {}
      "pb" => {
        println!();
        print_bitboard(board.get_sides()[0] | board.get_sides()[1]);
        println!();
        print_bitboard(board.get_sides()[0]);
        println!();
        print_bitboard(board.get_sides()[1])
      }
      "test" => {
        println!("{:?}", board.get_sides());
      }
      "quit" => exit(0),
      _ => println!("Error unknown command: {}", command),
    }
  }
}

#[cfg(test)]
mod tests {
  use engine::engine::Engine;
  use lib::{alph_to_pos, pos_to_alph};
  use movegen::movegen::MoveGen;
  use parsers::{fen::Fen, time::TimerKeeper};
  use structs::Sides;
  use Iridium::bitcount;

  use super::*;

  #[test]
  fn test_eval() {
    let test_fen: Vec<&str> = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1"
      .split(" ")
      .collect();

    let result = Engine::evaluate(Fen::from_fen(&test_fen).unwrap());
    assert_eq!(result, 0.0);
  }

  #[test]
  fn test_movegen() {
    let test_fen: Vec<&str> = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1"
      .split(" ")
      .collect();

    let result = MoveGen::gen_moves(Fen::from_fen(&test_fen).unwrap(), Sides::BLACK, true);
    assert_eq!(result, vec![]);
  }

  #[test]
  fn test_time() {
    let command: Vec<&str> = "go wtime 300000 btime 300000 winc 0 binc 0"
      .split(" ")
      .collect();

    let res = Time::parse_time(&command[0..]);

    assert_eq!(
      res,
      Constraints {
        time: Some(TimerKeeper {
          time_msec: [300000, 300000],
          inc_msec: [0, 0],
          mtg: 0
        }),
        depth: None,
        nodes: None,
        mate: None,
        movetime: None,
        infinite: false,
        ponder: false
      }
    );
  }

  #[test]
  fn test_alph_to_pos() {
    assert_eq!(alph_to_pos("e5"), Ok(36))
  }

  #[test]
  fn test_pos_to_alph() {
    assert_eq!(pos_to_alph(36), Ok("e5".to_string()))
  }

  #[test]
  fn test_bitcount() {
    assert_eq!(8.0, bitcount(Board::default().bb_pieces[0][0].0));
  }

  // #[test]
  // fn test_apply_move() {
  //   let mut tboard: Board = Board::default();

  //   print_bitboard(tboard.bb_pieces[0][Pieces::KNIGHT as usize]);

  //   println!();
  //   tboard.apply_move(Move { start: 1, dest: 18, capture: None });

  //   print_bitboard(tboard.bb_pieces[0][Pieces::KNIGHT as usize]);
  // }
}

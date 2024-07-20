use std::process::exit;
use engine::engine::Engine;
use fen::parsers::Parsers;

mod lib;
mod fen;
mod engine;
mod movegen;

fn ucimode() {
  // Identification
  println!("id name {}", lib::NAME);
  println!("id author {}", lib::AUTHOR);
  // Options 

  // Ready
  println!("uciok");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  loop {
      let command = lib::get_input("");
      match command.as_str() {
            "uci" => ucimode(),
            "isready" => {println!("readyok")},
            "setoption" => {},
            "register" => {},
            "ucinewgame" => {},
            "position" => {println!("{}", Engine::evaluate(Parsers::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")?))},
            "go" => {},
            "stop" => {},
            "ponder" => {},
            "ponderhint" => {},
            "quit" => exit(0),
            _ => println!("Error unknown command: {}", command),
      }
  }
}

#[cfg(test)]
mod tests {
  use movegen::{movegen::MoveGen, structs::Move};

use super::*;

  #[test]
  fn test_eval() {
    let result = Engine::evaluate(Parsers::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1").unwrap());  
    assert_eq!(result, 0);
  }

  #[test]
  fn test_movegen() {
    let result = MoveGen::gen_moves(Parsers::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1").unwrap(), lib::Sides::WHITE);
    let no_moves: Vec<Move> = vec![];
    assert_eq!(result, no_moves);
  }
}


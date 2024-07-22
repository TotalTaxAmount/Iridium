use std::{env::Args, process::exit};
use fen::parsers::{Constraints, Parsers};
use Iridium::Board;

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
  let mut board: Board;
  let mut constraints: Constraints;

  loop {
    let input = lib::get_input("");
    let args: Vec<&str> = input.split(" ").collect();
    let command: &str = args.first().unwrap_or(&"");
    match command {
          "uci" => ucimode(),
          "isready" => {println!("readyok")},
          "setoption" => {},
          "register" => {},
          "ucinewgame" => {},
          "position" => {
            if args.len() < 2 {
              println!("Invalid arguments");
              continue;
            }

            if args[1] == "startpos" {
              board = Board::default();
              continue;
            }

            match Parsers::from_fen(&&args[1..7]) {
                Ok(b) => { board = b },
                Err(e) => {println!("{}", e)}
            } 
          },
          "go" => {
            constraints = Parsers::parse_time(&args);
            println!("{:?}", constraints);
          },
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
  use engine::engine::Engine;
use movegen::movegen::MoveGen;

  use super::*;


  #[test]
  fn test_eval() {
    let test_fen: Vec<&str> = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".split(" ").collect();
    
    let result = Engine::evaluate(Parsers::from_fen(&test_fen).unwrap());  
    assert_eq!(result, 0);
  }

  #[test]
  fn test_movegen() {
    let test_fen: Vec<&str> = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".split(" ").collect();

    let result = MoveGen::gen_moves(Parsers::from_fen(&test_fen).unwrap(), lib::Sides::WHITE);
    assert_eq!(result, vec![]);
  }
}


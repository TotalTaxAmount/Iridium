use std::process::exit;
use fen::parsers::Parsers;
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
  loop {
    let input = lib::get_input("");
    let command: Vec<&str> = input.split(" ").collect();
    match command[0] {
          "uci" => ucimode(),
          "isready" => {println!("readyok")},
          "setoption" => {},
          "register" => {},
          "ucinewgame" => {},
          "position" => {
            if command.len() < 7 {
              println!("Invalid arguments");
              continue;
            }

            match Parsers::from_fen(&command[1..7]) {
                Ok(b) => { board = b },
                Err(e) => {println!("{}", e)}
            } 
          },
          "go" => {},
          "stop" => {},
          "ponder" => {},
          "ponderhint" => {},
          "quit" => exit(0),
          _ => println!("Error unknown command: {}", command.concat()),
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


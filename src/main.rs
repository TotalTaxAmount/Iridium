use std::process::exit;
use fen::parsers::Parsers;
use Iridium::move_gen;

mod lib;
mod fen;

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
            "position" => {println!("{}", Parsers::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")?)},
            "test" => {println!("{:?}", move_gen::bishop_moves(9))},
            "go" => {},
            "stop" => {},
            "ponder" => {},
            "ponderhint" => {},
            "quit" => exit(0),
            _ => println!("Error unknown command: {}", command),
      }
  }
}


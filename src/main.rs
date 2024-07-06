use std::process::exit;

mod lib;
mod fen;
mod consts;

fn ucimode() {
  // Identification
  println!("id name {}", consts::NAME);
  println!("id author {}", consts::AUTHOR);
  // Options 

  // Ready
  println!("uciok");
}

fn main() {
  
  loop {
      let command = lib::get_input("");
      match command.as_str() {
            "uci" => ucimode(),
            "isready" => {println!("readyok")},
            "setoption" => {},
            "register" => {},
            "ucinewgame" => {},
            "position" => {},
            "go" => {},
            "stop" => {},
            "ponder" => {},
            "ponderhint" => {},
            "quit" => exit(0),
            _ => println!("Error unknown command: {}", command),
      }
  }
}


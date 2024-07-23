use crate::structs::Board;

use crate::lib::alph_to_pos;

use super::fen::Fen;

struct Position;
impl Position {
  pub fn parse_position(args: &[&str]) -> Board {
    let mut board: Board = Board::default();
    let mut token_id = 0;
    while let Some(t) =  args.get(token_id) {
      match *t {
        "startpos" => {
          continue;
        },
        "fen" => {
          if let Ok(b) = Fen::from_fen(&args[(token_id + 1)..(token_id + 6)]) {
            board = b;
          } else {
            println!("Error parsing fen");
            return Board::default();
          }
        },
        "moves" => {
          for m in args[(token_id + 1)..].iter() {
            let (start_str, end_str) = m.split_at(2);
              let start = alph_to_pos(start_str);
              let end = alph_to_pos(end_str);
              
              for sides in board.bb_pieces {
                for pieces in sides {
                  
                }
              }
          }
    
        },
        _ => continue  
      }
      token_id += 1;
    }

    board
  }
}
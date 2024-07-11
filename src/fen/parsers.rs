use std::{fmt::Error, num::ParseIntError};

use crate::lib::{Board, Sides, Pieces, BitBoard};

pub struct Parsers;
impl Parsers {
    pub fn from_fen(fen: &str) -> Result<Board, Error> {
      let parts: Vec<_> = fen.split(" ").collect();
      
      let pos = Self::parse_position(parts[0]);
      let side_to_play = Self::parse_side_to_play(parts[1]);
      let castle_rights = Self::parse_castling(parts[2]);
      let en_passant = Self::parse_en_passant(parts[3]);
      let halfmoves = Self::parse_halfmoves(parts[4]);
      let fullmoves = Self::parse_fullmoves(parts[5]);

      Ok(Board {
        turn: side_to_play?,
        bb_pieces: pos?,
        white_can_oo: castle_rights?.0,
        black_can_oo: castle_rights?.1,
        white_can_ooo: castle_rights?.2,
        black_can_ooo: castle_rights?.3,
        en_passant_square: en_passant?,
        half_moves: halfmoves?,
        full_moves: fullmoves?,
      })
    }

    fn parse_position(part: &str) -> Result<[[BitBoard; 6]; 2], Error> {
      let ranks: Vec<_> = part.split("/").collect();
      let mut placement: [[BitBoard; 6]; 2] = [
        [BitBoard(0); 6],
        [BitBoard(0); 6]
      ];
    
      if ranks.len() != 8 {
        return Err(Error)
      }

      for (rank, pieces) in ranks.iter().rev().enumerate() {
        let mut file = 0;

        for piece_char in pieces.chars() {
          match piece_char.to_digit(10) {
              Some(n) => file += n as usize,
              None => {
                let piece = Pieces::from_char(piece_char).ok_or(Error);
                let bit_mask = rank * 8 + file;

                placement[piece.clone().unwrap().0 as usize][piece.unwrap().1 as usize].0 |= 1u64 << bit_mask;
                file += 1;
              }
            }
          }
        }
        Ok(placement)
    }

    fn parse_en_passant(part: &str) -> Result<Option<BitBoard>, Error> {
      if part == "-" {
        return Ok(None);
      }

      if part.len() != 2 {
        return Err(Error);
      }

      let chars: Vec<_> = part.chars().collect();
      let (file, rank) = (chars[0], chars[1]);
      println!("{}{}", rank, file);

      let file = match file {
          'a' => 0,
          'b' => 1,
          'c' => 2,
          'd' => 3,
          'e' => 4,
          'f' => 5,
          'g' => 6,
          'h' => 7,
          _ => 0,
      };


      let rank = match rank.to_digit(10) {
        Some(n) => u64::from(n) - 1,
        None => return Ok(None), 
      };

      Ok(Some(BitBoard(1u64 << rank * 8 + file)))
    }

    fn parse_side_to_play(part: &str) -> Result<Sides, Error> {
      match part {
          "w" => Ok(Sides::WHITE),
          "b" => Ok(Sides::BLACK),
          _ => Err(Error)
      }
    }

    fn parse_castling(part: &str) -> Result<(bool, bool, bool, bool), Error> {
      Ok((
        part.contains("K"),
        part.contains("Q"),
        part.contains("k"),
        part.contains("q")
      ))
    }

    fn parse_fullmoves(part: &str) -> Result<u64, Error> {
      match part.parse() {
          Ok(n) => Ok(n),
          Err(_) => Err(Error),
      }
    }
    
    fn parse_halfmoves(part: &str) -> Result<u64, Error> {
      match part.parse() {
          Ok(n) => Ok(n),
          Err(_) => Err(Error),
      }
    }

  
}
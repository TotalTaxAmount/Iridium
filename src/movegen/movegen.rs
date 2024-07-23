use crate::structs::{BitBoard, Board, Move, Pieces, Sides};


pub struct MoveGen;
impl MoveGen {
  pub fn gen_moves(board: Board, side: Sides) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for (s, position) in board.bb_pieces.into_iter().enumerate() {
      if Sides::from_usize(s) != Some(side) {
        continue;
      } 
      for (piece, bb) in position.into_iter().enumerate() {
        match Pieces::from_usize(piece) {
            Some(Pieces::PAWN) => moves.append(&mut Self::pawn_moves(bb, board.en_passant_square)),
            Some(Pieces::BISHOP) => moves.append(&mut Self::bishop_moves(bb)),
            Some(Pieces::KNIGHT) => moves.append(&mut Self::knight_moves(bb)),
            Some(Pieces::ROOK) => moves.append(&mut Self::rock_moves(bb)),
            Some(Pieces::QUEEN) => moves.append(&mut Self::queen_moves(bb)),
            Some(Pieces::KING) => moves.append(&mut Self::king_moves(bb, 
              if Sides::from_usize(s) == Some(Sides::WHITE) {
                board.white_can_oo
              } else {
                board.black_can_oo
              },
              if Sides::from_usize(s) == Some(Sides::WHITE) {
                board.white_can_ooo
              } else {
                board.black_can_ooo
              })),
            
            None => continue
        }
      }
    
    }

    moves
  }

  pub fn pawn_moves(bb: BitBoard, en_passant: Option<u8>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }
  
  pub fn bishop_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn knight_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn rock_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn queen_moves(bb: BitBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }

  pub fn king_moves(bb: BitBoard, castle_oo: bool, castle_ooo: bool) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    moves
  }
}
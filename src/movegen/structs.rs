use Iridium::Pieces;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
  start: u8,
  dest: u8,
  piece: Pieces,
  capture: bool,
}
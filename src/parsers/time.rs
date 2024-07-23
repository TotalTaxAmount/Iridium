use core::fmt;
use std::{env::args, error, fmt::{write, Error}};

use crate::structs::{Board, Sides, Pieces, BitBoard};



#[derive(PartialEq, Debug)]
pub struct TimerKeeper {
  pub time_msec: [i32; 2],
  pub inc_msec: [i32; 2],
  pub mtg: u32,
}

impl TimerKeeper {
    fn new() -> Self {
      TimerKeeper {
        time_msec: [0; 2],
        inc_msec: [0; 2],
        mtg: 0,
      }
    }

    fn blank(&self) -> bool {
      self.time_msec != [0; 2] &&
      self.inc_msec != [0; 2] &&
      self.mtg != 0
    }
}

impl fmt::Display for TimerKeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "time: [ w: {}ms b: {}ms ] inc: [ w: {}ms b: {}ms ] moves to go: {}", 
        &self.time_msec[0],
        &self.time_msec[1],
        &self.inc_msec[0],
        &self.inc_msec[1],
        &self.mtg
      )
    }
}

#[derive(PartialEq, Debug)]
pub struct Constraints {
  pub time: Option<TimerKeeper>,
  pub depth: Option<u32>, 
  pub nodes: Option<u32>,
  pub mate: Option<u32>,
  pub movetime: Option<u32>,
  pub infinite: bool,
  pub ponder: bool
}

impl Constraints {
    pub fn new() -> Self {
      Constraints {
        time: Some(TimerKeeper::new()),
        depth: None,
        nodes: None,
        mate: None,
        movetime: None,
        infinite: false,
        ponder: false
      }
    }
}

pub struct Time;
impl Time {
    pub fn parse_time(time_args: &[&str]) -> Constraints{
      let mut token_id = 0;
      let mut constraints = Constraints::new();
      let mut time = TimerKeeper::new();
      while let Some(t) = time_args.get(token_id) {
        match *t {
          "infinite" => {
            constraints.infinite = true;
          },

          "ponder" => {
            constraints.ponder = true;
          },

          "wtime" => {
            if let Some(wtime) = time_args.get(token_id + 1) {
              if let Ok(t) = wtime.parse::<i32>() {
                time.time_msec[0] = t;
              }
              token_id += 1;
            }
          },

          "btime" => {
            if let Some(btime) = time_args.get(token_id + 1) {
              if let Ok(t) = btime.parse::<i32>() {
                time.time_msec[1] = t;
              }
              token_id += 1;
            }
          },

          "winc" => {
            if let Some(winc) = time_args.get(token_id + 1) {
              if let Ok(t) = winc.parse::<i32>() {
                time.inc_msec[0] = t;
              }
              token_id += 1;
            }
          },

          "binc" => {
            if let Some(binc) = time_args.get(token_id + 1) {
              if let Ok(t) = binc.parse::<i32>() {
                time.inc_msec[1] = t;
              }
              token_id += 1;
            }
          },

          "movestogo" => {
            if let Some(mtg) = time_args.get(token_id + 1) {
              if let Ok(t) = mtg.parse::<u32>() {
                time.mtg = t;
              }
              token_id += 1;
            }
          },

          "depth" => {
            if let Some(depth) = time_args.get(token_id + 1) {
              if let Ok(t) = depth.parse::<u32>() {
                constraints.depth = Some(t);
              }
              token_id += 1;
            }
          },

          "nodes" => {
            if let Some(nodes) = time_args.get(token_id + 1) {
              if let Ok(t) = nodes.parse::<u32>() {
                constraints.nodes = Some(t);
              }

              token_id += 1;
            }
          },

          "mate" => {
            if let Some(mate) = time_args.get(token_id + 1) {
              if let Ok(t) = mate.parse::<u32>() {
                constraints.mate = Some(t);
              }

              token_id += 1;
            }
          },

          "movetime" => {
            if let Some(movetime) = time_args.get(3) {
              if let Ok(t) = movetime.parse::<u32>() {
                constraints.movetime = Some(t);
              }
              token_id += 1;
            }
          }
          
          _ => {},
        }
        token_id += 1;
      }
      if !time.blank() {
        constraints.time = Some(time);
      }

      constraints
    }
}
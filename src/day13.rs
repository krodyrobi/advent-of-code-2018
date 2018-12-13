use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Cart {
  position: Position,
  direction: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Position {
  x: usize,
  y: usize,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

fn read_input(input: &str) -> (Vec<Cart>, HashMap<Position, usize>) {
  input.lines();
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Position {
  Position { x: 1, y: 1 }
}


#[aoc(day13, part2)]
pub fn part2(input: &str) -> Position {
  Position { x: 1, y: 1 }
}


#[cfg(test)]
mod tests {
  use super::{part1, Position};

  #[test]
  fn sample1() {
    assert_eq!(Position { x: 7, y: 3 }, part1(
      "/->-\\\n\
    |   |  /----\\\n\
    | /-+--+-\\  |\n\
    | | |  | v  |\n\
    \\-+-/  \\-+--/\n\
    \\------/\n"
    ));
  }
}
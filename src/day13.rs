use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct Cart {
  position: Position,
  direction: usize,
  orientation: usize,
}

impl PartialEq for Cart {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position
  }
}

impl Ord for Cart {
  fn cmp(&self, other: &Self) -> Ordering {
    let x = self.position.x;
    let ox = other.position.x;
    let y = self.position.y;
    let oy = other.position.y;

    (y, x).cmp(&(oy, ox))
  }
}

impl PartialOrd for Cart {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
  x: usize,
  y: usize,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}

const LEFT_INDEX: usize = 0;
const UP_INDEX: usize = 1;
const STRAIGHT_INDEX: usize = UP_INDEX;
const RIGHT_INDEX: usize = 2;
const DOWN_INDEX: usize = 3;

const LEFT: (isize, isize) = (-1, 0);
const UP: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (1, 0);
const DOWN: (isize, isize) = (0, 1);

const DIRECTION_DELTA: &'static [(isize, isize); 3] = &[LEFT, UP, RIGHT];
const ORIENTATION_DELTA: &'static [(isize, isize); 4] = &[LEFT, UP, RIGHT, DOWN];


fn read_input(input: &str) -> (Vec<Cart>, HashMap<Position, char>) {
  let mut carts = vec!();

  let grid = input.lines()
    .enumerate()
    .flat_map(|(row, line)| {
      line.chars()
        .enumerate()
        .filter_map(|(column, c)| {
          if c == ' ' {
            return None;
          }

          let position = Position { x: row, y: column };

          if c == '>' || c == '<' {
            carts.push(Cart {
              position,
              direction: LEFT_INDEX,
              orientation: match c {
                '>' => RIGHT_INDEX,
                _ => LEFT_INDEX
              },
            });
            return Some((position, '-'));
          }

          if c == '^' || c == 'v' {
            carts.push(Cart {
              position,
              direction: LEFT_INDEX,
              orientation: match c {
                '^' => UP_INDEX,
                _ => DOWN_INDEX
              },
            });
            return Some((position, '|'));
          }

          Some((position, c))
        })
        .collect::<HashMap<Position, char>>()
    })
    .collect::<HashMap<Position, char>>();

  (carts, grid)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Position {
  println!("{:?}", read_input(input));
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
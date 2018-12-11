use std::fmt;

const DIRECTIONS: [(i32, i32); 9] = [
  (-1, -1), (0, -1), (1, -1),
  (-1, 0), (0, 0), (1, 0),
  (-1, 1), (0, 1), (1, 1)
];

#[derive(Debug, Clone)]
pub struct Chunk {
  x: i32,
  y: i32,
  score: i32,
}

impl Chunk {
  pub fn new(x: i32, y: i32, grid_serial: i32) -> Chunk {
    let mut score = 0;
    for (dx, dy) in &DIRECTIONS {
      score += Chunk::score(x + dx, y + dy, grid_serial);
    }

    Chunk { x, y, score }
  }

  fn score(x: i32, y: i32, grid_serial: i32) -> i32 {
    let rack_id = x + 10;
    ((rack_id * y + grid_serial) * rack_id) / 100 % 10 - 5i32
  }

  pub fn top_left(&self) -> (i32, i32) {
    (self.x-1, self.y -1)
  }
}

impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Center {},{} Score: {} Top-left: {:?} ", self.x, self.y, self.score, self.top_left())
  }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Chunk {
  let grid_serial = input.lines().next().unwrap().parse::<i32>().unwrap();

  let mut chunks = vec!();
  for chunk_x in 2..300 {
    for chunk_y in 2..300 {
      chunks.push(Chunk::new(chunk_x, chunk_y, grid_serial));
    }
  }

  chunks.sort_unstable_by_key(|c| c.score);
  chunks.last().unwrap().clone()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u32 {
  1
}


#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1() {
    let chunk = part1("18");
    assert_eq!((33,45), chunk.top_left());
    assert_eq!(29, chunk.score);
  }

  #[test]
  fn sample2() {
    let chunk = part1("42");
    assert_eq!((21,61), chunk.top_left());
    assert_eq!(30, chunk.score);
  }

//  #[test]
//  fn sample2() {
//    assert_eq!(66, part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"))
//  }
}
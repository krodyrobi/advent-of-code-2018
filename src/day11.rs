use std::fmt;


#[derive(Debug, Clone)]
pub struct Chunk {
  x: i32,
  y: i32,
  size: i32,
  score: i32,
}

impl Chunk {
  pub fn new(x: i32, y: i32, grid_serial: i32, size: i32) -> Chunk {
    let mut score = 0;
    for dx in 0..size {
      for dy in 0..size {
        score += Chunk::score(x + dx, y + dy, grid_serial);
      }
    }

    Chunk { x, y, size, score }
  }

  fn score(x: i32, y: i32, grid_serial: i32) -> i32 {
    let rack_id = x + 10;
    ((rack_id * y + grid_serial) * rack_id) / 100 % 10 - 5i32
  }
}

impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Top-left: {},{} Score: {} Size: {}", self.x, self.y, self.score, self.size)
  }
}

pub fn workhorse(input: &str, size: i32) -> Chunk {
  let grid_serial = input.lines().next().unwrap().parse::<i32>().unwrap();

  let mut chunks = vec!();
  for chunk_x in 1..=301-size {
    for chunk_y in 1..=301-size {
      chunks.push(Chunk::new(chunk_x, chunk_y, grid_serial, size));
    }
  }

  chunks.sort_unstable_by_key(|c| c.score);
  chunks.last().unwrap().clone()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Chunk {
  workhorse(input, 3)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> Chunk {
  // naive approach 150s +  luckily it converges way sooner <20 size so the loop should be 20ish
  let mut chunks = vec!();
  for i in 1..=300 {
    chunks.push(workhorse(input, i))
  }

  chunks.sort_unstable_by_key(|c| c.score);
  chunks.last().unwrap().clone()
}


#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1() {
    let chunk = part1("18");
    assert_eq!((33,45), (chunk.x, chunk.y));
    assert_eq!(29, chunk.score);
  }

  #[test]
  fn sample2() {
    let chunk = part1("42");
    assert_eq!((21,61), (chunk.x, chunk.y));
    assert_eq!(30, chunk.score);
  }

  #[test]
  #[ignore]
  fn sample3() {
    let chunk = part2("18");
    assert_eq!((90,269), (chunk.x, chunk.y));
    assert_eq!(16, chunk.size);
  }

  #[test]
  #[ignore]
  fn sample4() {
    let chunk = part2("42");
    assert_eq!((232,251), (chunk.x, chunk.y));
    assert_eq!(12, chunk.size);
  }
}
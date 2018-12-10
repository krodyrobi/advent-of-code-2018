use regex::Regex;
use std::collections::{HashMap, HashSet};

#[macro_export]
macro_rules! cap {
  ( $caps:expr, $i:expr ) => {
    {
      $caps.get($i).unwrap().as_str().parse::<u32>().unwrap()
    }
  };
}

fn work_horse(input: &str) -> HashMap<(u32, u32), HashSet<u32>> {
  let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
  let mut visited: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();

  input.lines()
    .map(|line| {
      let caps = re.captures(line).unwrap();
      let id = cap!(caps, 1);
      let x = cap!(caps, 2);
      let y = cap!(caps, 3);
      let lx = cap!(caps, 4);
      let ly = cap!(caps, 5);

      (id, x, y, lx, ly)
    })
    .for_each(|(id, x, y, lx, ly)| {
      for xs in x..x + lx {
        for ys in y..y + ly {
          visited.entry((xs, ys)).or_default().insert(id);
        }
      }
    });

  visited
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
  work_horse(input).values()
    .filter(|set| set.len() >= 2)
    .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
  let data = work_horse(input);
  let (possible, overlapping): (Vec<_>, Vec<_>) = data.values()
    .partition(|ids| ids.len() < 2);

  let possible: HashSet<_> = possible.into_iter().flatten().collect();
  let overlapping: HashSet<_> = overlapping.into_iter().flatten().collect();

  **possible.difference(&overlapping)
    .next()
    .unwrap()
}


#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1() {
    assert_eq!(4, part1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"))
  }

  #[test]
  fn sample2() {
    assert_eq!(
      3,
      part2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"))
  }
}
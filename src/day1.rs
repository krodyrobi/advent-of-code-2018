use std::iter::Iterator;
use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
  numbers(input).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
  let mut set = HashSet::new();
  let mut result = 0;

  set.insert(result);
  for num in numbers(input).cycle() {
    result += num;

    if set.contains(&result) {
      return result;
    } else {
      set.insert(result);
    }
  }

  unreachable!();
}

fn numbers<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a + Clone {
    input
      .lines()
      .map(|line| line.parse().unwrap())
}

#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1_1() {
    assert_eq!(3, part1("+1\n-2\n+3\n+1"))
  }

  #[test]
  fn sample1_2() {
    assert_eq!(3, part1("+1\n+1\n+1"))
  }

  #[test]
  fn sample1_3() {
    assert_eq!(0, part1("+1\n+1\n-2"))
  }


  #[test]
  fn sample1_4() {
    assert_eq!(-6, part1("-1\n-2\n-3"))
  }


  #[test]
  fn sample2_1() {
    assert_eq!(2, part2("+1\n-2\n+3\n+1"))
  }

  #[test]
  fn sample2_2() {
    assert_eq!(0, part2("+1\n-1"))
  }

  #[test]
  fn sample2_3() {
    assert_eq!(10, part2("+3\n+3\n+4\n-2\n-4"))
  }

  #[test]
  fn sample2_4() {
    assert_eq!(5, part2("-6\n+3\n+8\n+5\n-6"))
  }

  #[test]
  fn sample2_5() {
    assert_eq!(14, part2("+7\n+7\n-2\n-7\n-4"))
  }
}
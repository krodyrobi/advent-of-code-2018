use std::collections::VecDeque;

struct Digits {
  n: usize,
  divisor: usize,
}

impl Digits {
  fn new(n: usize) -> Self {
    let mut divisor = 1;
    while n >= divisor * 10 {
      divisor *= 10;
    }

    Digits {
      n,
      divisor,
    }
  }
}

impl Iterator for Digits {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.divisor == 0 {
      None
    } else {
      let v = Some(self.n / self.divisor);
      self.n %= self.divisor;
      self.divisor /= 10;
      v
    }
  }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> String {
  let recipes = input.lines().next().unwrap_or("0").parse().unwrap_or(0 as usize);
  let mut board: VecDeque<usize> = VecDeque::new();
  board.push_back(3);
  board.push_back(7);

  let mut first_index = 0 as usize;
  let mut second_index = 1 as usize;

  while board.len() <= recipes + 10 {
    let a = board[first_index];
    let b = board[second_index];

    Digits::new(a + b).into_iter().for_each(|digit| board.push_back(digit));
    first_index += a + 1;
    first_index %= board.len();
    second_index += b + 1;
    second_index %= board.len();
  }

  board.into_iter()
    .skip(recipes)
    .take(10)
    .map(|d| d.to_string())
    .collect::<Vec<_>>().join("")
}


#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
  let input = input.lines().next().unwrap_or("0");

  let mut board: VecDeque<usize> = VecDeque::new();
  board.push_back(3);
  board.push_back(7);

  let mut first_index = 0 as usize;
  let mut second_index = 1 as usize;
  let mut buffer = String::from("37");
  let mut buffer_last_index = 2;

  loop {
    let a = board[first_index];
    let b = board[second_index];

    Digits::new(a + b)
      .into_iter()
      .for_each(|digit| {
        board.push_back(digit);
        buffer += &digit.to_string();
        buffer_last_index += 1;
      });

    first_index += a + 1;
    first_index %= board.len();
    second_index += b + 1;
    second_index %= board.len();

    if let Some(i) = buffer.find(input) {
      return buffer_last_index - buffer.len() + i;
    }

    if buffer.len() > input.len() + 2 {
      buffer = String::from(&buffer[2..])
    }
  }
}


#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1() {
    assert_eq!("0124515891", part1("5"));
  }

  #[test]
  fn sample2() {
    assert_eq!("5158916779", part1("9"));
  }

  #[test]
  fn sample3() {
    assert_eq!("9251071085", part1("18"));
  }

  #[test]
  fn sample4() {
    assert_eq!("5941429882", part1("2018"));
  }

  #[test]
  fn sample5() {
    assert_eq!(5, part2("01245"));
  }

  #[test]
  fn sample6() {
    assert_eq!(9, part2("51589"));
  }

  #[test]
  fn sample7() {
    assert_eq!(18, part2("92510"));
  }

  #[test]
  fn sample8() {
    assert_eq!(2018, part2("59414"));
  }
}
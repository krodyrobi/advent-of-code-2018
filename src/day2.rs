use std::iter::Iterator;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::fmt;
use std::collections::HashSet;


#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
  let (twos, threes): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|word| {
      let mut frequency: HashMap<char, u32> = HashMap::new();
      for c in word.chars() {
        *frequency.entry(c).or_default() += 1;
      }

      frequency
    })
    .map(|frequencies| {
      let twos = frequencies
        .iter()
        .any(|(_, value)| *value == 2);

      let threes = frequencies
        .iter()
        .any(|(_, value)| *value == 3);

      (twos, threes)
    })
    .unzip();

  let twos = twos.into_iter().filter(|v| *v).count();
  let threes = threes.into_iter().filter(|v| *v).count();
  twos * threes
}

#[derive(Debug)]
pub struct MissingChar {
  string: String,
  index: usize
}

impl MissingChar {
  fn chars<'a>(&'a self) -> impl Iterator<Item = char> + 'a {
    self.string
      .char_indices()
      .filter_map(move |(index, char)| {
        if index != self.index {
          Some(char)
        } else {
          None
        }
      })
  }
}

impl Eq for MissingChar {}

impl PartialEq for MissingChar  {
  fn eq(&self, other: &Self) -> bool {
    self.chars().zip(other.chars()).all(|(l, r)| l == r)
  }
}

impl Hash for MissingChar {
  fn hash<H>(&self, state: &mut H)
    where
      H: Hasher,
  {
    for (index, char) in self.string.char_indices() {
      if index != self.index {
        (index, char).hash(state);
      }
    }
  }
}

impl fmt::Display for MissingChar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let string: String = self.chars().collect();
    write!(f, "{}", string)
  }
}


#[aoc(day2, part2)]
pub fn part2(input: &str) -> MissingChar {
  let mut missing_set = HashSet::new();

  for line in input.lines().into_iter() {
    for i in 0..(line.len()) {
      let missing = MissingChar {
        string: String::from(line),
        index: i
      };

      if missing_set.contains(&missing) {
        return missing;
      } else {
        missing_set.insert(missing);
      }
    }
  }

  unreachable!()
}


#[cfg(test)]
mod tests {
  use super::{part1, part2, MissingChar};

  #[test]
  fn sample1() {
    assert_eq!(12, part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"))
  }

  #[test]
  fn sample2() {
    assert_eq!(
      MissingChar { string: String::from("fguij"), index: 2},
      part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"))
  }
}
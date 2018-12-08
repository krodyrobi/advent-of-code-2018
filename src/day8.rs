use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Node {
  metas: Vec<u32>,
  nodes: Vec<Node>,
}


impl Node {
  fn build(numbers: &mut impl Iterator<Item=u32>) -> Node {
    let node_count = numbers.next().unwrap();
    let meta_count = numbers.next().unwrap();

    let mut node = Node {
      metas: vec![],
      nodes: vec![],
    };

    for _ in 0..node_count {
      node.nodes.push(Self::build(numbers));
    }

    for _ in 0..meta_count {
      node.metas.push(numbers.next().unwrap());
    }

    node
  }

  fn visit<F, J, T>(&self, f: &mut F, j: &mut J) -> T
    where F: FnMut(&Node) -> T,
          J: FnMut(T, T) -> T
  {
    let result = f(&self);
    self.nodes.iter().fold(result, |acc, el| {
      let result = el.visit(f, j);
      j(acc, result)
    })
  }
}

impl FromStr for Node {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut numbers = s.split(' ')
      .map(|c| c.parse::<u32>().unwrap());

    Ok(Node::build(&mut numbers))
  }
}

fn seriously(node: &Node) -> u32 {
  if node.nodes.is_empty() {
    node.metas.iter().sum()
  } else {
    node.metas
      .iter()
      .map(|index| {
        node.nodes.get((index - 1) as usize).map_or(0u32, seriously)
      })
      .sum()
  }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
  let node: Node = input.parse().unwrap();
  node.visit(&mut |node: &Node| node.metas.iter().sum(), &mut |a, b| a + b)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
  let node: Node = input.parse().unwrap();
  seriously(&node)
}


#[cfg(test)]
mod tests {
  use super::{part1, part2};

  #[test]
  fn sample1() {
    assert_eq!(138, part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"))
  }

  #[test]
  fn sample2() {
    assert_eq!(66, part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"))
  }
}
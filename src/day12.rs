use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Pot {
  index: i32,
  value: bool,
}


fn left_pad(list: &mut VecDeque<Pot>) {
  let first_pot = list.pop_front().unwrap();
  let first_pot_index = first_pot.index;
  list.push_front(first_pot);
  list.push_front(Pot { index: first_pot_index - 1, value: false });
  list.push_front(Pot { index: first_pot_index - 2, value: false });
  list.push_front(Pot { index: first_pot_index - 3, value: false });
  list.push_front(Pot { index: first_pot_index - 4, value: false });
}

fn right_pad(list: &mut VecDeque<Pot>) {
  let last_pot = list.pop_back().unwrap();
  let last_pot_index = last_pot.index;
  list.push_back(last_pot);
  list.push_back(Pot { index: last_pot_index + 1, value: false });
  list.push_back(Pot { index: last_pot_index + 2, value: false });
  list.push_back(Pot { index: last_pot_index + 3, value: false });
  list.push_back(Pot { index: last_pot_index + 4, value: false });
}

pub fn generation(list: &mut VecDeque<Pot>, rules: &HashMap<Vec<bool>, bool>) -> VecDeque<Pot> {
  left_pad(list);
  right_pad(list);

  let mut new_generation = list.clone();
  let vec = list.iter().map(|pot| pot.value).collect::<Vec<_>>();

  for (i, window) in vec.windows(5).enumerate() {
    let mut pot = new_generation.get_mut(i + 2).unwrap();
    pot.value = *rules.get(window).unwrap_or(&false);
  }

  new_generation
}

fn read_input(input: &str) -> (VecDeque<Pot>, HashMap<Vec<bool>, bool>) {
  let mut lines = input.lines();
  let initial = lines.next().unwrap().split(" ").nth(2).unwrap();
  let mut list: VecDeque<Pot> = VecDeque::new();

  for (i, c) in initial.chars().enumerate() {
    list.push_back(Pot { index: i as i32, value: c == '#' })
  }

  lines.next();

  let mut rules = HashMap::new();
  for line in lines {
    let mut parts = line.split(" ");
    let rule_in = parts.nth(0).unwrap().chars().take(5).map(|c| c == '#').collect::<Vec<_>>();
    let rule_out = parts.nth(1).unwrap() == "#";

    rules.insert(rule_in, rule_out);
  }

  (list, rules)
}

fn produce_sum(list: &VecDeque<Pot>) -> i32 {
  list
    .iter()
    .filter_map(|pot| {
      if pot.value {
        Some(pot.index)
      } else {
        None
      }
    })
    .sum()
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
  let (mut list, rules) = read_input(input);

  for _ in 0..20 {
    list = generation(&mut list, &rules);
  }

  produce_sum(&mut list)
}


#[aoc(day12, part2)]
pub fn part2(input: &str) -> i64 {
  let (mut list, rules) = read_input(input);
  let mut g = 0;

  loop {
    let new_list = generation(&mut list, &rules);
    let is_same = list.iter()
      .skip_while(|pot| !pot.value)
      .zip(new_list.iter().skip_while(|pot| !pot.value))
      .all(|(a, b)| a.value == b.value);

    g += 1;

    if is_same {
      let start = produce_sum(&list) as i64;
      let end = produce_sum(&new_list) as i64;

      break (start..)
        .step_by((end - start) as usize)
        .nth(50_000_000_000 - g + 1)
        .unwrap();
    }

    list = new_list;
  }
}


#[cfg(test)]
mod tests {
  use super::part1;

  #[test]
  fn sample1() {
    assert_eq!(325, part1(
      "initial state: #..#.#..##......###...###\n\
      \n\
      ...## => #\n\
      ..#.. => #\n\
      .#... => #\n\
      .#.#. => #\n\
      .#.## => #\n\
      .##.. => #\n\
      .#### => #\n\
      #.#.# => #\n\
      #.### => #\n\
      ##.#. => #\n\
      ##.## => #\n\
      ###.. => #\n\
      ###.# => #\n\
      ####. => #\n"
    ));
  }
}
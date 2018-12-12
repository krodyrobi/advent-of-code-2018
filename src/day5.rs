extern crate regex;

use self::regex::Regex;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
  let re = Regex::new(r"(aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz)").unwrap();

  let mut before = input.to_string();
  loop {
    let after = re.replace_all(&before, "").to_string();
    if before == after {
      break
    }

    before = after
  }

  before.len()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
  (('a' as u32)..=('z' as u32))
    .into_iter()
    .map(|c| {
      let c = std::char::from_u32(c).unwrap();
      let data = input
        .chars()
        .filter( |target|
          *target != c && *target != c.to_ascii_uppercase())
        .collect::<String>();

      part1(&data)
    })
    .min()
    .unwrap()
}


#[cfg(test)]
mod tests {
  use super::part1;

  #[test]
  fn sample1() {
    assert_eq!(10, part1("dabAcCaCBAcCcaDA"))
  }

  #[test]
  fn sample2() {
    assert_eq!(0, part1("abcdefghHGFEDCBA"))
  }
}
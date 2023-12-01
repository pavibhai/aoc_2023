use std::iter::Iterator;

pub fn part1(lines: &[Vec<char>]) -> u32 {
  lines.iter().map(|line| {
    let n = 10 * line.iter()
      .find(|&c| c.is_ascii_digit())
      .unwrap()
      .to_digit(10).unwrap_or(0);

    n + line.iter()
      .rfind(|&c| c.is_ascii_digit())
      .unwrap()
      .to_digit(10).unwrap_or(0)
  }).sum()
}

fn to_digit(input: &[char], first: bool, numbers: &[Vec<char>]) -> Option<usize> {
  let inp_len = input.len();
  if first {
    match input.last() {
      None => None,
      Some(c)  if c.is_ascii_digit() => c.to_digit(10).map(|n| n as usize),
      _ => numbers.iter().position(|num| {
        let l = num.len();
        inp_len >= l && num == &input[inp_len - l..inp_len]
      })
    }
  } else {
    match input.first() {
      None => None,
      Some(c) if c.is_ascii_digit() => c.to_digit(10).map(|n| n as usize),
      _ => numbers.iter().position(|num| {
        let l = num.len();
        inp_len >= l && num == &input[0..l]
      })
    }
  }
}

fn make_numbers() -> [Vec<char>; 10] {
  [
    "zero".chars().collect(),
    "one".chars().collect(),
    "two".chars().collect(),
    "three".chars().collect(),
    "four".chars().collect(),
    "five".chars().collect(),
    "six".chars().collect(),
    "seven".chars().collect(),
    "eight".chars().collect(),
    "nine".chars().collect(),
  ]
}

pub fn part2(lines: &[Vec<char>]) -> usize {
  let numbers: [Vec<char>; 10] = make_numbers();
  lines.iter().map(|line| {
    let indicies: Vec<usize> = (0..line.len()).collect();
    let n = 10 * indicies.iter().find_map(|i| {
      to_digit(&line[0..i + 1], true, &numbers)
    }).unwrap_or(0);
    n + indicies.iter().rev().find_map(|i| {
      to_digit(&line[*i..line.len()], false, &numbers)
    }).unwrap_or(0)
  }).sum()
}

pub fn generator(input: &str) -> Vec<Vec<char>> {
  input.lines()
    .map(|l| l.chars().collect())
    .collect()
}

#[cfg(test)]
mod tests {
  use crate::day1::{generator, make_numbers, part1, part2, to_digit};

  fn str_to_digit(input: &str, first: bool, numbers: &[Vec<char>]) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    to_digit(&chars, first, numbers)
  }

  fn input() -> String {
    [
      "1abc2",
      "pqr3stu8vwx",
      "a1b2c3d4e5f",
      "treb7uchet",
    ].join("\n")
  }

  fn input2() -> String {
    [
      "two1nine",
      "eightwothree",
      "abcone2threexyz",
      "xtwone3four",
      "4nineeightseven2",
      "zoneight234",
      "7pqrstsixteen",
    ].join("\n")
  }

  #[test]
  fn test_generator() {
    let values = generator(&input());
    assert_eq!(values.len(), 4);
    assert_eq!(values[0].len(), 5);
    assert_eq!(values[1].len(), 11);
    assert_eq!(values[2].len(), 11);
    assert_eq!(values[3].len(), 10);
  }

  #[test]
  fn test_part_1() {
    let values = generator(&input());
    assert_eq!(142, part1(&values));
  }

  #[test]
  fn test_to_digit() {
    let numbers: [Vec<char>; 10] = make_numbers();
    assert_eq!(1, str_to_digit("one", true, &numbers).unwrap());
    assert_eq!(1, str_to_digit("one", false, &numbers).unwrap());
    assert_eq!(None, str_to_digit("onea", true, &numbers));
    assert_eq!(1, str_to_digit("one1", false, &numbers).unwrap());
    assert_eq!(9, str_to_digit("two1nine", true, &numbers).unwrap());
    assert_eq!(2, str_to_digit("two1nine", false, &numbers).unwrap());
  }

  #[test]
  fn test_part2() {
    let lines = generator(&input2());
    assert_eq!(281, part2(&lines));
  }
}
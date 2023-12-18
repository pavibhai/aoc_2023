use crate::day13::Loc::{ASH, ROCK};

#[derive(Debug, Eq, PartialEq)]
enum Loc {
  ASH,
  ROCK,
}

impl Loc {
  fn from(c: char) -> Loc {
    match c {
      '.' => ASH,
      '#' => ROCK,
      _ => panic!("Unexpected space {c}"),
    }
  }
}

pub struct Pattern {
  layout: Vec<Vec<Loc>>,
}

impl Pattern {
  fn from(input: &str) -> Pattern {
    let map = input.lines().map(|line| line.chars().map(Loc::from).collect()).collect();
    Pattern { layout: map }
  }

  fn height(&self) -> u32 {
    self.layout.len() as u32
  }

  fn width(&self) -> u32 {
    self.layout[0].len() as u32
  }

  fn score(&self, smudges: u32) -> u32 {
    match self.check_for_vertical_mirror(smudges) {
      Some(v) => v,
      _ => 100 * self.check_for_horizontal_mirror(smudges).unwrap(),
    }
  }

  fn row_equal_count(&self, r1: u32, r2: u32) -> u32 {
    (0..self.layout[0].len()).filter(|c| {
      self.layout[r1 as usize][*c] == self.layout[r2 as usize][*c]
    }).count() as u32
  }

  fn col_equal_count(&self, c1: u32, c2: u32) -> u32 {
    self.layout.iter().filter(|row| row[c1 as usize] == row[c2 as usize])
      .count() as u32
  }

  fn check_for_vertical_mirror(&self, smudges: u32) -> Option<u32> {
    // Find the adjacent columns that are identical
    for c1 in 0..self.width() - 1 {
      let mut smudges = smudges;
      match self.height() - self.col_equal_count(c1, c1 + 1) {
        v if v > smudges => continue,
        v => smudges -= v,
      }
      let mut x1 = c1;
      let mut x2 = c1 + 1;
      let mut matched = true;
      while x1 > 0 && x2 + 1 < self.width() {
        x1 -= 1;
        x2 += 1;
        match self.height() - self.col_equal_count(x1, x2) {
          v if v > smudges => {
            matched = false;
            break;
          }
          v => smudges -= v,
        }
      }
      if matched && smudges == 0 { return Some(c1 + 1); }
    }
    None
  }

  fn check_for_horizontal_mirror(&self, smudges: u32) -> Option<u32> {
    // Find the adjacent rows that are identical
    for r1 in 0..self.height() - 1 {
      let mut smudges = smudges;
      match self.width() - self.row_equal_count(r1, r1 + 1) {
        v if v > smudges => {
          continue;
        }
        v => {
          smudges -= v
        }
      }
      let mut y1 = r1;
      let mut y2 = r1 + 1;
      let mut matched = true;
      while y1 > 0 && y2 + 1 < self.height() {
        y1 -= 1;
        y2 += 1;
        match self.width() - self.row_equal_count(y1, y2) {
          v if v > smudges => {
            matched = false;
            break;
          }
          v => smudges -= v,
        }
      }
      if matched && smudges == 0 { return Some(r1 + 1); }
    }
    None
  }
}

pub fn generator(input: &str) -> Vec<Pattern> {
  input.split("\n\n")
    .map(Pattern::from)
    .collect()
}

pub fn part1(patterns: &[Pattern]) -> u32 {
  patterns.iter().map(|p| {
    p.score(0)
  }).sum()
}

pub fn part2(patterns: &[Pattern]) -> u32 {
  patterns.iter().map(|p| {
    p.score(1)
  }).sum()
}

#[cfg(test)]
mod tests {
  use crate::day13::{generator, part1, part2, Pattern};

  fn input() -> String {
    "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".to_string()
  }

  #[test]
  fn test_generator() {
    let patterns = generator(&input());
    assert_eq!(2, patterns.len());
  }

  #[test]
  fn test_mirrors() {
    let pattern = Pattern::from("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.");
    assert_eq!(5, pattern.check_for_vertical_mirror(0).unwrap());
    assert_eq!(None, pattern.check_for_horizontal_mirror(0));
  }

  #[test]
  fn test_boundaries() {
    let pattern = Pattern::from("#.......##.
.#.###....#
###...#....
..#........
###.##...#.
.#..##..###
####.#.##.#
...#.#.###.
..##.#.###.
.#...#.##.#
#.##..#####
#.##..#####
.#...#.##.#
..##.#.###.
...#.#.###.");
    assert_eq!(None, pattern.check_for_vertical_mirror(0));
    assert_eq!(Some(11), pattern.check_for_horizontal_mirror(0));

    let pattern = Pattern::from("##...#.#..#
....##.####
#####.#.##.
#####......
.#######..#
######..##.
..##.######");
    assert_eq!(Some(9), pattern.check_for_vertical_mirror(0));
    assert_eq!(None, pattern.check_for_horizontal_mirror(0));

    let pattern = Pattern::from("....##.####
##..#.#..#.
##..#...#..
..#..####..
..###.#####
##..#.#.#..
####..#####
..#..#####.
..##.#####.
##..###..#.
#####.#####
...##....##
....##..#.#
#####.##..#
...##.##..#
...##.#...#
#####.##..#");
    assert_eq!(10, pattern.row_equal_count(14, 15));
    assert_eq!(Some(15), pattern.check_for_horizontal_mirror(1));
  }

  #[test]
  fn test_part1() {
    let patterns = generator(&input());
    assert_eq!(405, part1(&patterns));
  }

  #[test]
  fn test_part2() {
    let patterns = generator(&input());
    assert_eq!(400, part2(&patterns));
  }
}
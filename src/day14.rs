use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::day14::Space::{CUBE, EMPTY, ROUND};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Space {
  EMPTY,
  ROUND,
  CUBE,
}

impl Space {
  fn from(c: char) -> Space {
    match c {
      '.' => EMPTY,
      'O' => ROUND,
      '#' => CUBE,
      _ => panic!("Unexpected space {c}"),
    }
  }

  fn to_char(&self) -> char {
    match self {
      EMPTY => '.',
      ROUND => 'O',
      CUBE => '#',
    }
  }
}

impl Display for Space {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_char())
  }
}

#[derive(Clone)]
pub struct Platform {
  layout: Vec<Vec<Space>>,
}

impl Platform {
  fn from(input: &str) -> Platform {
    let layout = input.lines()
      .map(|line| line.chars().map(Space::from).collect()).collect();

    Platform {
      layout,
    }
  }

  fn find_cycle(&mut self) -> (HashMap<Vec<(usize, usize)>, usize>, (usize, usize)) {
    let mut repeats: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut i = 0;
    loop {
      let north_weights = self.north_weights();
      if repeats.contains_key(&north_weights) {
        let prev = *repeats.get(&north_weights).unwrap();
        return (repeats, (prev, i));
      }
      repeats.insert(north_weights, i);
      self.cycle();
      i += 1;
    }
  }

  fn cycle(&mut self) -> Vec<usize> {
    self.tilt_north();
    self.tilt_west();
    self.tilt_south();
    self.tilt_east()
  }

  fn north_weights(&self) -> Vec<(usize, usize)> {
    (0..self.width()).map(|c| {
      self.layout.iter().enumerate()
        .fold((0, 0), |(count, weight), (r, row)| {
          if row[c] == ROUND {
            (count + 1, weight + self.layout.len() - r)
          } else {
            (count, weight)
          }
        })
    }).collect()
  }

  fn tilt_north(&mut self) -> Vec<usize> {
    self.tilt_north_south(false)
  }

  fn tilt_south(&mut self) -> Vec<usize> {
    self.tilt_north_south(true)
  }

  fn tilt_north_south(&mut self, reverse: bool) -> Vec<usize> {
    let (start, change) = if reverse { (self.height() - 1, -1_i32) } else { (0, 1) };
    let mut sweep_edge: Vec<usize> = vec![start; self.width()];

    let mut process_row = |r: usize, p: &mut Platform| {
      for c in 0..p.width() {
        match &p.layout[r][c] {
          EMPTY => {}
          CUBE => sweep_edge[c] = (r as i32 + change) as usize,
          ROUND => {
            p.layout[r][c] = EMPTY;
            p.layout[sweep_edge[c]][c] = ROUND;
            sweep_edge[c] = (sweep_edge[c] as i32 + change) as usize;
          }
        }
      }
    };
    if reverse {
      (0..self.height()).rev().for_each(|r| process_row(r, self))
    } else {
      (0..self.height()).for_each(|r| process_row(r, self))
    }

    sweep_edge
  }

  fn tilt_east(&mut self) -> Vec<usize> {
    self.tilt_west_east(true)
  }

  fn tilt_west(&mut self) -> Vec<usize> {
    self.tilt_west_east(false)
  }

  fn tilt_west_east(&mut self, reverse: bool) -> Vec<usize> {
    let (start, change) = if reverse { (self.width() - 1, -1_i32) } else { (0, 1) };
    let mut sweep_edge: Vec<usize> = vec![start; self.height()];

    let mut process_col = |c: usize, p: &mut Platform| {
      for r in 0..p.height() {
        match &p.layout[r][c] {
          EMPTY => {}
          CUBE => sweep_edge[r] = (c as i32 + change) as usize,
          ROUND => {
            p.layout[r][c] = EMPTY;
            p.layout[r][sweep_edge[r]] = ROUND;
            sweep_edge[r] = (sweep_edge[r] as i32 + change) as usize;
          }
        }
      }
    };
    if reverse {
      (0..self.height()).rev().for_each(|r| process_col(r, self))
    } else {
      (0..self.height()).for_each(|r| process_col(r, self))
    }

    sweep_edge
  }

  fn width(&self) -> usize {
    self.layout[0].len()
  }

  fn height(&self) -> usize {
    self.layout.len()
  }
}

fn north_weight(north_weights: &[(usize, usize)]) -> usize {
  north_weights.iter().map(|(_, w)| w).sum()
}

impl Display for Platform {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    for row in &self.layout {
      for s in row {
        output.push(s.to_char());
      }
      output.push('\n');
    }
    output.pop();
    write!(f, "{output}")
  }
}

pub fn generator(input: &str) -> Platform {
  Platform::from(input)
}

pub fn part1(platform: &Platform) -> usize {
  // Tilt north
  let mut sweep_edge: Vec<usize> = vec![platform.layout.len(); platform.width()];
  let mut score = 0_u32;
  for (r, row) in platform.layout.iter().enumerate() {
    for c in 0..platform.width() {
      match &row[c] {
        EMPTY => {}
        CUBE => sweep_edge[c] = platform.height() - 1 - r,
        ROUND => {
          score += sweep_edge[c] as u32;
          sweep_edge[c] -= 1;
        }
      }
    }
  }
  score as usize
}

pub fn part2(platform: &Platform) -> usize {
  let (map, (start, repeat)) = platform.clone().find_cycle();
  let final_state: usize = start + (1000000000_usize - start) % (repeat - start);
  for (k, v) in map {
    if v == final_state { return north_weight(&k); }
  }
  panic!("Could not find the matching ")
}

#[cfg(test)]
mod tests {
  use crate::day14::{generator, north_weight, part1, part2};
  use crate::day14::Space::{EMPTY, ROUND};

  fn input() -> String {
    "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string()
  }

  #[test]
  fn test_generator() {
    let p = generator(&input());
    assert_eq!(ROUND, p.layout[0][0]);
    assert_eq!(EMPTY, p.layout[9][9]);
  }

  #[test]
  fn test_part1() {
    let mut p = generator(&input());
    assert_eq!(136, part1(&p));
    p.tilt_north_south(false);
    assert_eq!(136, north_weight(&p.north_weights()));
  }

  #[test]
  fn test_cycle() {
    let mut p = generator(&input());
    p.tilt_north();
    assert_eq!(p.to_string(), "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....");
    p.tilt_west();
    assert_eq!(p.to_string(), "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....");
    p.tilt_south();
    assert_eq!(p.to_string(), ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....");
    p.tilt_east();
    assert_eq!(p.to_string(), ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....");

    p.cycle();
    assert_eq!(p.to_string(), ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O");

    p.cycle();
    assert_eq!(p.to_string(), ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O");
  }

  #[test]
  fn test_find_cycle() {
    let mut p = generator(&input());
    assert_eq!((3, 10), p.find_cycle().1);

    let mut p = generator(&input());
    for _ in 0..11 {
      p.cycle();
    }
  }

  #[test]
  fn test_part2() {
    let p = generator(&input());
    assert_eq!(64, part2(&p));
  }
}
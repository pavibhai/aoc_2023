use std::fmt::{Display, Formatter};
use std::mem::swap;
use crate::day16::Space::{Empty, Mirror, Splitter};

const LEFT_ENTRY: u8 = 1;
const RIGHT_ENTRY: u8 = 2;
const UP_ENTRY: u8 = 4;
const DOWN_ENTRY: u8 = 8;

#[derive(Debug, Eq, PartialEq)]
enum Space {
  Empty,
  Mirror(i8),
  Splitter(i8, i8),
}

impl Space {
  fn from(input: char) -> Space {
    match input {
      '.' => Empty,
      '/' => Mirror(-1),
      '\\' => Mirror(1),
      '-' => Splitter(1, 0),
      '|' => Splitter(0, 1),
      _ => panic!("Unexpected character {input}")
    }
  }

  fn to_char(&self) -> char {
    match self {
      Empty => '.',
      Mirror(-1) => '/',
      Mirror(1) => '\\',
      Splitter(1, 0) => '-',
      Splitter(0, 1) => '|',
      _ => panic!("Unexpected space")
    }
  }
}

impl Display for Space {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_char())
  }
}


pub fn generator(input: &str) -> Contraption {
  Contraption::from(input)
}

pub fn part1(contraption: &Contraption) -> u32 {
  let start = Position { x: 0, y: 0, delta_x: 1, delta_y: 0 };
  count_energized(contraption, start)
}

fn count_energized(contraption: &Contraption, start: Position) -> u32 {
  let energized = contraption.simulate(start);
  energized.iter().map(|row| row.iter().filter(|&v| v > &0).count() as u32)
    .sum()
}

pub fn part2(contraption: &Contraption) -> u32 {
  let mut max_energized = 0;
  for x in 0..contraption.width() {
    max_energized = max_energized.max(count_energized(contraption,
                                                      Position { x, y: 0, delta_x: 0, delta_y: 1 }));
    max_energized = max_energized.max(count_energized(contraption,
                                                      Position { x, y: contraption.height() - 1, delta_x: 0, delta_y: -1 }));
  }
  for y in 0..contraption.height() {
    max_energized = max_energized.max(count_energized(contraption,
                                                      Position { x: 0, y, delta_x: 1, delta_y: 0 }));
    max_energized = max_energized.max(count_energized(contraption,
                                                      Position { x: contraption.width() - 1, y, delta_x: -1, delta_y: 0 }));
  }
  max_energized
}

#[derive(Clone)]
struct Position {
  x: i32,
  y: i32,
  delta_x: i8,
  delta_y: i8,
}

impl Position {
  fn forward(&mut self) {
    self.x += self.delta_x as i32;
    self.y += self.delta_y as i32;
  }

  fn entry(&self) -> u8 {
    match (self.delta_x, self.delta_y) {
      (1, _) => LEFT_ENTRY,
      (-1, _) => RIGHT_ENTRY,
      (_, 1) => UP_ENTRY,
      (_, -1) => DOWN_ENTRY,
      _ => panic!("Unexpected direction ({}, {})", self.delta_x, self.delta_y),
    }
  }

  fn reflect(&mut self, mirror: &i8) {
    swap(&mut self.delta_x, &mut self.delta_y);
    self.delta_y *= mirror;
    self.delta_x *= mirror;
  }

  fn split(&mut self, x: &i8, y: &i8) -> Option<Position> {
    if self.delta_x.abs() == *x || self.delta_y.abs() == *y {
      // Split aligned with direction, no change
      return None;
    }

    match (x, y) {
      (1, 0) => {
        self.delta_y = 0;
        self.delta_x = -1;
        Some(Position { x: self.x, y: self.y, delta_x: 1, delta_y: 0 })
      }
      (0, 1) => {
        self.delta_y = -1;
        self.delta_x = 0;
        Some(Position { x: self.x, y: self.y, delta_x: 0, delta_y: 1 })
      }
      _ => {
        panic!("Unexpected state, cannot split ({x},{y})")
      }
    }
  }
}

pub struct Contraption {
  grid: Vec<Vec<Space>>,
}

impl Contraption {
  fn from(input: &str) -> Contraption {
    let grid = input.lines()
      .map(|line| line.chars().map(Space::from).collect())
      .collect();
    Contraption { grid }
  }

  fn height(&self) -> i32 {
    self.grid.len() as i32
  }

  fn width(&self) -> i32 {
    self.grid[0].len() as i32
  }

  fn is_valid(&self, pos: &Position) -> bool {
    pos.x >= 0 && pos.x < self.width() && pos.y >= 0 && pos.y < self.height()
  }

  fn simulate(&self, start: Position) -> Vec<Vec<u8>> {
    let mut energized: Vec<Vec<u8>> = vec![vec![0; self.width() as usize]; self.height() as usize];
    let mut stack: Vec<Position> = Vec::new();
    stack.push(start);

    while !stack.is_empty() {
      let mut p = stack.pop().unwrap();

      if !self.is_valid(&p)
        || energized[p.y as usize][p.x as usize] & p.entry() == p.entry() {
        // If entry is invalid or already seen then skip processing
        continue;
      }
      energized[p.y as usize][p.x as usize] |= p.entry();
      let space = &self.grid[p.y as usize][p.x as usize];
      match space {
        Empty => {}
        Mirror(sgn) => {
          p.reflect(sgn);
        }
        Splitter(x, y) => {
          if let Some(mut additional) = p.split(x, y) {
            additional.forward();
            stack.push(additional);
          }
        }
      }
      p.forward();
      stack.push(p)
    }

    energized
  }
}

#[cfg(test)]
mod tests {
  use crate::day16::{generator, part1};

  fn input() -> String {
    ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....".to_string()
  }

  #[test]
  fn test_generator() {
    let c = generator(&input());
    assert_eq!(c.height(), 10);
    assert_eq!(c.width(), 10);
  }

  #[test]
  fn test_part1() {
    let c = generator(&input());
    assert_eq!(46, part1(&c))
  }
}

use std::collections::HashSet;
use crate::day10::Direction::{EAST, NORTH, SOUTH, WEST};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

const DIRECTIONS: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];
const GROUND: [bool; 4] = [false; 4];
const EAST_WEST: [bool; 4] = [false, false, true, true];
const NORTH_SOUTH: [bool; 4] = [true, true, false, false];

type Connects = [bool; 4];

pub struct Field {
  start: XY,
  map: Vec<Vec<Connects>>,
}

impl Field {
  fn connects(&self, x: &usize, y: &usize) -> &Connects {
    &self.map[*y][*x]
  }
  fn width(&self) -> usize {
    self.map.first().unwrap().len()
  }
  fn connect_north(&self, xy: &XY) -> Option<(XY, Direction)> {
    if xy.y > 0 {
      match &self.map[xy.y - 1][xy.x] {
        [_, true, _, _] => Some((XY { x: xy.x, y: xy.y - 1 }, SOUTH)),
        _ => None,
      }
    } else {
      None
    }
  }

  fn connect_south(&self, xy: &XY) -> Option<(XY, Direction)> {
    if xy.y + 1 < self.map.len() {
      match &self.map[xy.y + 1][xy.x] {
        [true, _, _, _] => {
          Some((XY { x: xy.x, y: xy.y + 1 }, NORTH))
        }
        _ => None,
      }
    } else {
      None
    }
  }

  fn connect_east(&self, xy: &XY) -> Option<(XY, Direction)> {
    if xy.x + 1 < self.width() {
      match &self.map[xy.y][xy.x + 1] {
        [_, _, _, true] => {
          Some((XY { x: xy.x + 1, y: xy.y }, WEST))
        }
        _ => None,
      }
    } else {
      None
    }
  }

  fn connect_west(&self, xy: &XY) -> Option<(XY, Direction)> {
    if xy.x > 0 {
      match &self.map[xy.y][xy.x - 1] {
        [_, _, true, _] => Some((XY { x: xy.x - 1, y: xy.y }, EAST)),
        _ => None,
      }
    } else {
      None
    }
  }

  fn compute_perimeter(&self) -> Vec<XY> {
    let mut perimeter: Vec<XY> = Vec::new();
    perimeter.push(self.start);
    let (mut curr, mut skip_dir) = match self.connect_north(&self.start) {
      Some(xy) => xy,
      _ => match self.connect_south(&self.start) {
        Some(xy) => xy,
        _ => match self.connect_east(&self.start) {
          Some(xy) => xy,
          _ => panic!("Could not determine a valid connection")
        }
      }
    };

    while curr != self.start {
      perimeter.push(curr);
      for (connects, to_dir) in self.connects(&curr.x, &curr.y).iter().zip(DIRECTIONS) {
        // Skip if there is no connection or incoming direction
        if !connects || to_dir == skip_dir { continue; }
        let v = match to_dir {
          NORTH => self.connect_north(&curr),
          SOUTH => self.connect_south(&curr),
          WEST => self.connect_west(&curr),
          EAST => self.connect_east(&curr),
        };
        if let Some((c, dir)) = v {
          curr = c;
          skip_dir = dir;
          break;
        }
      }
    }
    perimeter
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct XY {
  x: usize,
  y: usize,
}

pub fn generator(input: &str) -> (Field, Vec<XY>) {
  let mut start: Option<XY> = None;
  let map: Vec<Vec<Connects>> = input.lines().enumerate().map(|(y, line)| {
    line.chars().enumerate().map(|(x, c)| {
      match c {
        '|' => [true, true, false, false], // North South
        '-' => [false, false, true, true], // East West
        'L' => [true, false, true, false], // North East
        'J' => [true, false, false, true], // North West
        '7' => [false, true, false, true], // South West
        'F' => [false, true, true, false], // South East
        '.' => [false; 4],
        'S' => {
          start = Some(XY { x, y });
          [true; 4]
        }
        _ => panic!("Unexpected character {c}"),
      }
    }).collect()
  }).collect();

  if start.is_none() {
    panic!("Could not determine start");
  }

  let mut f = Field {
    start: start.unwrap(),
    map,
  };


  f.map[f.start.y][f.start.x][NORTH as usize] = f.connect_north(&f.start).is_some();
  f.map[f.start.y][f.start.x][SOUTH as usize] = f.connect_south(&f.start).is_some();
  f.map[f.start.y][f.start.x][EAST as usize] = f.connect_east(&f.start).is_some();
  f.map[f.start.y][f.start.x][WEST as usize] = f.connect_west(&f.start).is_some();
  let p = f.compute_perimeter();
  (f, p)
}

pub fn part1(puzzle: &(Field, Vec<XY>)) -> u32 {
  ((puzzle.1.len() + 1) / 2) as u32
}

pub fn part2(puzzle: &(Field, Vec<XY>)) -> u32 {
  // compute area
  let field = &puzzle.0;
  let perimeter: HashSet<&XY> = HashSet::from_iter(puzzle.1.iter());
  let mut x_min = field.width();
  let mut x_max = 0;
  let mut y_min = field.map.len();
  let mut y_max = 0;

  // Compute max bounds
  for xy in &perimeter {
    x_min = x_min.min(xy.x);
    x_max = x_max.max(xy.x);
    y_min = y_min.min(xy.y);
    y_max = y_max.max(xy.y);
  }

  let mut area = 0;
  let mut prev;
  let mut include;
  let mut curr;
  for y in y_min..=y_max {
    prev = &GROUND;
    include = false;
    for x in x_min..=x_max {
      let xy = XY { x, y };
      curr = field.connects(&x, &y);
      if perimeter.contains(&xy) {
        match curr {
          &NORTH_SOUTH => {
            include = !include;
            prev = &NORTH_SOUTH;
          }
          &EAST_WEST => {}
          c if c[NORTH as usize] == prev[NORTH as usize] => {
            include = !include;
            prev = c;
          }
          _ => {}
        }
      } else {
        if include { area += 1 }
      }
    }
  }
  area
}

#[cfg(test)]
mod tests {
  use crate::day10::{generator, part1, part2, XY};

  fn input() -> String {
    ".....
.S-7.
.|.|.
.L-J.
.....".to_string()
  }

  #[test]
  fn test_generator() {
    let (f, _) = generator(&input());
    assert_eq!(5, f.map.first().unwrap().len());
  }

  #[test]
  fn test_directions() {
    let (f, _) = generator("..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
    let n = f.connect_north(&XY { x: 3, y: 1 });
    assert!(n.is_some());
  }

  #[test]
  fn test_part1() {
    /*    let f = generator(&input());
        assert_eq!(4, part1(&f));*/

    let f = generator("..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
    assert_eq!(8, part1(&f));
  }

  #[test]
  fn test_part2() {
    let f = generator(".....
.S-7.
.|.|.
.L-J.
.....");
    assert_eq!(1, part2(&f));

    let f = generator("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");
    assert_eq!(4, part2(&f));

    let f = generator("..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........");
    assert_eq!(4, part2(&f));

    let f = generator(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...");
    assert_eq!(8, part2(&f));

    let f = generator("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
    assert_eq!(10, part2(&f));
  }
}
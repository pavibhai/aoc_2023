use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::day21::Plot::{Garden, Rock};

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn generator(input: &str) -> Puzzle {
  Puzzle::from(input)
}

pub fn part1(puzzle: &Puzzle) -> u64 {
  puzzle.walk_to_end_positions(64, &puzzle.start, false)
}

pub fn part2(puzzle: &Puzzle) -> u64 {
  puzzle.walk_optimized(26501365)
}

#[derive(Debug, Eq, PartialEq)]
enum Plot {
  Garden,
  Rock,
}

#[derive(Default, Eq, PartialEq, Debug, Clone, Hash)]
struct XY {
  x: i32,
  y: i32,
}

impl XY {
  fn create(x: i32, y: i32) -> XY {
    XY { x, y }
  }
}

pub struct Puzzle {
  start: XY,
  map: Vec<Vec<Plot>>,
}

impl Puzzle {
  fn height(&self) -> u32 {
    self.map.len() as u32
  }

  fn width(&self) -> u32 {
    self.map[0].len() as u32
  }

  fn from(input: &str) -> Puzzle {
    let mut start = XY::default();

    let map = input.lines().enumerate()
      .map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
          match c {
            '#' => Rock,
            '.' => Garden,
            'S' => {
              start.x = x as i32;
              start.y = y as i32;
              Garden
            }
            _ => panic!("Unexpected plot {c}")
          }
        }).collect()
      }).collect();
    let p = Puzzle { start, map };
    assert_eq!(p.height(), p.width());
    p
  }

  fn walk(&self, steps: u32, start: &XY, limit: bool) -> Vec<u32> {
    let mut visited: HashMap<XY, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<Entry>> = BinaryHeap::new();
    heap.push(Reverse(Entry::from(start.x, start.y, 0)));

    while !heap.is_empty() {
      let entry = heap.pop().unwrap().0;
      if entry.steps > steps || visited.contains_key(&entry.xy) { continue; }
      visited.insert(entry.xy.clone(), entry.steps);

      for (dx, dy) in &NEIGHBORS {
        let x = entry.xy.x + dx;
        let y = entry.xy.y + dy;
        if limit
          && (x < 0 || x as u32 == self.width()
          || y < 0 || y as u32 == self.height()) {
          continue;
        }

        let y_pos = y.rem_euclid(self.height() as i32) as usize;
        let x_pos = x.rem_euclid(self.width() as i32) as usize;
        if self.map[y_pos][x_pos] == Rock {
          continue;
        }

        heap.push(Reverse(Entry::from(x, y, entry.steps + 1)));
      }
    }

    visited.values().copied().collect()
  }

  fn walk_positions(&self, steps: u32) -> ((u32, u32), (u32, u32)) {
    let start_state = steps % 2;
    let positions = self.walk(steps, &self.start, true);
    let mut start_pos = 0;
    let mut max_start_steps = 0;
    let mut next_pos = 0;
    let mut max_next_steps = 0;
    for position in positions {
      if position % 2 == start_state {
        start_pos += 1;
        max_start_steps = max_start_steps.max(position);
      } else {
        next_pos += 1;
        max_next_steps = max_next_steps.max(position);
      }
    }
    ((start_pos, max_start_steps), (next_pos, max_next_steps))
  }

  fn walk_to_end_positions(&self, steps: u32, start: &XY, limit: bool) -> u64 {
    let req_state = steps % 2;
    let positions = self.walk(steps, start, limit);
    positions.iter().filter(|&v| v % 2 == req_state).count() as u64
  }

  fn compute_positions(&self, steps: u32, start: &XY) -> u64 {
    let positions = self.walk_to_end_positions(steps,
                                               start,
                                               true);
    if steps < self.height() {
      positions
    } else {
      positions + self.compute_positions(steps - self.height(), start)
    }
  }

  fn compute_ends(&self, steps: u32) -> u64 {
    let mut positions: u64 = self.compute_positions(steps,
                                                    &XY::create(self.start.x,
                                                                (self.height() - 1) as i32));
    positions += self.compute_positions(steps,
                                        &XY { x: self.start.x, y: 0 });
    positions += self.compute_positions(steps,
                                        &XY { x: (self.width() - 1) as i32, y: self.start.y });
    positions += self.compute_positions(steps,
                                        &XY { x: 0, y: self.start.y }, );
    positions
  }

  fn walk_optimized(&self, steps: u32) -> u64 {
    if steps < self.height() {
      return self.walk_to_end_positions(steps, &self.start, false)
    }
    assert_eq!(self.height(), self.width());
    assert_eq!(self.start.x, self.start.y);
    let mut total_positions = 0_u64;
    let remainder = steps % self.height() + self.start.x as u32;
    total_positions += self.compute_ends(remainder);

    let ((start_pos, start_max), (next_pos, next_max)) = self.walk_positions(steps);
    assert!(start_max <= self.height());
    assert!(next_max <= self.height());
    let count = steps / self.height();
    if count > 1 {
      if count % 2 == 0 {
        total_positions += (count - 1) as u64 * (count - 1) as u64 * start_pos as u64;
        total_positions += count as u64 * count as u64 * next_pos as u64;
      } else {
        total_positions += (count - 1) as u64 * (count - 1) as u64 * next_pos as u64;
        total_positions += count as u64 * count as u64 * start_pos as u64;
      }
    } else {
      total_positions += start_pos as u64;
    }


    let side_remainder = remainder + self.start.x as u32;
    if remainder >= (self.start.x + 1) as u32 {
      let remaining_side_reminder = remainder - self.start.x as u32 - 1;
      total_positions += self.compute_positions(remaining_side_reminder,
                                                &XY::create((self.width() - 1) as i32,
                                                            (self.height() - 1) as i32));
      total_positions += self.compute_positions(remaining_side_reminder,
                                                &XY::create(0,
                                                            (self.height() - 1) as i32));
      total_positions += self.compute_positions(remaining_side_reminder,
                                                &XY::create((self.width() - 1) as i32,
                                                            0));
      total_positions += self.compute_positions(remaining_side_reminder,
                                                &XY::create(0,
                                                            0));
    }

    if count > 0 {
      total_positions += (count - 1) as u64 * self.compute_positions(side_remainder,
                                                                     &XY::create((self.width() - 1) as i32,
                                                                                 (self.height() - 1) as i32));
      total_positions += (count - 1) as u64 * self.compute_positions(side_remainder,
                                                                     &XY::create(0,
                                                                                 (self.height() - 1) as i32));
      total_positions += (count - 1) as u64 * self.compute_positions(side_remainder,
                                                                     &XY::create((self.width() - 1) as i32,
                                                                                 0));
      total_positions += (count - 1) as u64 * self.compute_positions(side_remainder,
                                                                     &XY::create(0,
                                                                                 0));
    }


    total_positions
  }
}

struct Entry {
  xy: XY,
  steps: u32,
}

impl Entry {
  fn from(x: i32, y: i32, steps: u32) -> Entry {
    Entry { xy: XY { x, y }, steps }
  }
}

impl Eq for Entry {}

impl Ord for Entry {
  fn cmp(&self, other: &Self) -> Ordering {
    self.steps.cmp(&other.steps)
  }
}

impl PartialEq<Self> for Entry {
  fn eq(&self, other: &Self) -> bool {
    self.steps == other.steps
  }
}

impl PartialOrd for Entry {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(test)]
mod tests {
  use crate::day21::{generator, XY};

  const UNOBSTRUCTED_INPUT: &str = "...........
......##.#.
.###..#..#.
..#.#...#..
....#.#....
.....S.....
.##......#.
.......##..
.##.#.####.
.##...#.##.
...........";
  const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

  #[test]
  fn test_generator() {
    let puzzle = generator(INPUT);
    assert_eq!(puzzle.start, XY { x: 5, y: 5 });
    assert_eq!(puzzle.width(), 11);
    assert_eq!(puzzle.height(), 11);
  }

  #[test]
  fn test_part1() {
    let puzzle = generator(INPUT);
    assert_eq!(16, puzzle.walk_to_end_positions(6, &puzzle.start, false));
  }

  #[test]
  fn test_part2() {
    let puzzle = generator(INPUT);
    assert_eq!(50, puzzle.walk_to_end_positions(10, &puzzle.start, false));
    assert_eq!(1594, puzzle.walk_to_end_positions(50, &puzzle.start, false));
    assert_eq!(6536, puzzle.walk_to_end_positions(100, &puzzle.start, false));
    assert_eq!(167004, puzzle.walk_to_end_positions(500, &puzzle.start, false));
    //assert_eq!(668697, puzzle.walk_to_end_positions(1000, &puzzle.start, false));
    //assert_eq!(16733044, puzzle.walk_to_end_positions(5000, &puzzle.start, false));
  }

  #[test]
  fn test_part2_unobstructed() {
    let puzzle = generator(UNOBSTRUCTED_INPUT);
    assert_eq!(90, puzzle.walk_optimized(10));
    assert_eq!(192, puzzle.walk_optimized(15));
    assert_eq!(1501, puzzle.walk_optimized(44));
    assert_eq!(1580, puzzle.walk_optimized(45));
    assert_eq!(1940, puzzle.walk_optimized(50));
    assert_eq!(7645, puzzle.walk_optimized(100));
    assert_eq!(7765, puzzle.walk_optimized(101));
    assert_eq!(188756, puzzle.walk_optimized(500));
    assert_eq!(753480, puzzle.walk_optimized(1000));
    assert_eq!(18807440, puzzle.walk_optimized(5000));
  }

  #[test]
  fn test_compute_ends() {
    let p = generator(UNOBSTRUCTED_INPUT);
    assert_eq!(162, p.compute_ends(11));

    let ((start_pos, start_max), (next_pos, next_max)) = p.walk_positions(50);
    assert_eq!(47, start_pos);
    assert_eq!(44, next_pos);
    assert_eq!(10, start_max);
    assert_eq!(9, next_max);
  }
}
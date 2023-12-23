use std::cmp::Ordering;
use std::mem::swap;

pub fn generator(input: &str) -> &str {
  input
}

fn to_direction_steps(input: &str) -> Vec<(char, u64)> {
  input.lines()
    .map(|line| {
      let mut splits = line.split_whitespace();
      (splits.next().unwrap().chars().next().unwrap(), splits.next().unwrap().parse().unwrap())
    }).collect()
}

pub fn part1(input: &str) -> u64 {
  let inputs: Vec<(char, u64)> = to_direction_steps(input);

  let plan = DigPlan::from(&inputs);
  plan.compute_area()
}

pub fn part2(input: &str) -> u64 {
  let inputs: Vec<(char, u64)> = input.lines().map(|line| {
    let (_, code) = line.rsplit_once(' ').unwrap();
    hex_to_step(code)
  }).collect();

  let plan = DigPlan::from(&inputs);
  plan.compute_area()
}


fn hex_to_step(input: &str) -> (char, u64) {
  let moves = u64::from_str_radix(&input[2..input.len() - 2], 16).unwrap();
  let mut chars = input.chars();
  chars.next_back();
  let direction = match chars.next_back().unwrap() {
    '0' => 'R',
    '1' => 'D',
    '2' => 'L',
    '3' => 'U',
    c => panic!("Unexpected direction {c}"),
  };
  (direction, moves)
}

pub struct DigPlan {
  edges: Vec<Edge>,
}

impl DigPlan {
  fn from(input: &[(char, u64)]) -> DigPlan {
    let mut x = 0;
    let mut y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut edges = Vec::new();
    for (c, m) in input {
      let (delta_x, delta_y) = char_to_deltas(c);
      edges.push(Edge { x1: x, y1: y, x2: x + (*m as i32 * delta_x), y2: y + (*m as i32 * delta_y) });
      x += *m as i32 * delta_x;
      y += *m as i32 * delta_y;
      min_x = min_x.min(x);
      min_y = min_y.min(y);
    }
    for edge in edges.iter_mut() {
      edge.x1 -= min_x;
      edge.x2 -= min_x;
      edge.y1 -= min_y;
      edge.y2 -= min_y;
      edge.make_1_min();
    }

    DigPlan { edges }
  }

  fn horizontal_edges(&self) -> Vec<&Edge> {
    let mut edges: Vec<&Edge> = self.edges.iter().filter(|&e| e.is_horizontal()).collect();
    edges.sort_by(|e1, e2| {
      match e1.y1.cmp(&e2.y2) {
        Ordering::Equal => e1.x1.cmp(&e2.x2),
        c => c
      }
    });
    edges
  }

  fn compute_area(&self) -> u64 {
    let mut area = 0_u64;
    let h_edges = self.horizontal_edges();
    let x_values = Edge::x_values(&h_edges);
    let mut includes = vec![false; x_values.len()];
    let mut prev_includes = includes.clone();
    let mut prev_y = -1;

    for edge in h_edges {
      if prev_y != edge.y1 {
        // Add current includes to prev_includes
        prev_includes.iter_mut().zip(&includes).for_each(|(p, v)| *p |= v);
        // Add the start edge which is the union of prev and current
        area += DigPlan::compute_row_include(&x_values, &prev_includes);
        // Add the details until the new edge
        area += DigPlan::compute_row_include(&x_values, &includes) * (edge.y1 - prev_y - 1) as u64;
        prev_includes = includes.clone();
        prev_y = edge.y1;
      }

      let start = x_values.binary_search(&(edge.x1 as u32)).unwrap();
      let end = x_values.binary_search(&(edge.x2 as u32)).unwrap();
      includes[start..end].iter_mut().for_each(|v| {
        *v ^= true;
      });
    }

    // Handle the last remaining edge
    prev_includes.iter_mut().zip(&includes).for_each(|(p, v)| if *v { *p = true });
    area += DigPlan::compute_row_include(&x_values, &prev_includes);
    area
  }

  fn compute_row_include(x_values: &[u32], includes: &[bool]) -> u64 {
    let mut row_include = 0_u64;
    let mut start_idx = 0_usize;
    let mut end_idx;
    while start_idx < includes.len() {
      for include in &includes[start_idx..] {
        if *include {
          break;
        } else {
          start_idx += 1;
        }
      }
      end_idx = start_idx;
      for include in &includes[start_idx..] {
        if *include {
          end_idx += 1;
        } else {
          break;
        }
      }
      if end_idx > start_idx {
        row_include += (x_values[end_idx] - x_values[start_idx] + 1) as u64;
      }
      start_idx = end_idx;
    }
    row_include
  }
}

#[derive(Debug, Eq, PartialEq)]
struct Edge {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}

impl Edge {
  fn x_values(edges: &[&Edge]) -> Vec<u32> {
    let mut x_values: Vec<u32> = edges.iter()
      .flat_map(|&e| [e.x1 as u32, e.x2 as u32])
      .collect();
    x_values.sort_unstable();
    x_values.dedup();

    x_values
  }

  fn is_horizontal(&self) -> bool {
    self.y1 == self.y2
  }

  fn make_1_min(&mut self) {
    if self.x1 > self.x2 && self.y1 == self.y2 {
      swap(&mut self.x1, &mut self.x2);
    } else if self.y1 > self.y2 && self.x1 == self.x2 {
      swap(&mut self.y1, &mut self.y2);
    } else if self.y1 != self.y2 && self.x1 != self.x2 {
      panic!("Expecting only horizontal or vertical lines but found {},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
  }
}

fn char_to_deltas(c: &char) -> (i32, i32) {
  match c {
    'R' => (1, 0),
    'L' => (-1, 0),
    'U' => (0, -1),
    'D' => (0, 1),
    _ => panic!("Unexpected direction {c}"),
  }
}

#[cfg(test)]
mod tests {
  use crate::day18::{DigPlan, Edge, generator, hex_to_step, part1, part2, to_direction_steps};

  fn input() -> String {
    "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".to_string()
  }

  #[test]
  fn test_generator() {
    let dp = DigPlan::from(&to_direction_steps(&input()));
    assert_eq!(dp.edges.len(), 14);
    assert_eq!(dp.horizontal_edges().len(), 7);
    assert_eq!(Edge::x_values(&dp.horizontal_edges()), vec![0, 1, 2, 4, 6]);
  }

  #[test]
  fn test_edge() {
    let mut e = Edge { x1: 5, y1: 2, x2: 2, y2: 2 };
    e.make_1_min();
    assert_eq!(Edge { x1: 2, x2: 5, y1: 2, y2: 2 }, e);

    let mut e = Edge { x1: 5, y1: 10, x2: 5, y2: 2 };
    e.make_1_min();
    assert_eq!(Edge { x1: 5, x2: 5, y1: 2, y2: 10 }, e);
  }

  #[test]
  fn test_part1() {
    let input = input();
    let dp = generator(&input);
    assert_eq!(62, part1(dp));
  }

  #[test]
  fn test_color_to_moves() {
    assert_eq!(('R', 461937), hex_to_step("(#70c710)"));
    assert_eq!(('D', 56407), hex_to_step("(#0dc571)"));
    assert_eq!(('R', 356671), hex_to_step("(#5713f0)"));

    let (_, code) = "U 2 (#7a21e3)".rsplit_once(' ').unwrap();
    assert_eq!(code, "(#7a21e3)");
  }

  #[test]
  fn test_part2() {
    let input = input();
    let dp = generator(&input);
    assert_eq!(952408144115, part2(dp));
  }
}
use std::collections::{HashMap, HashSet};
use crate::day23::Block::{Forest, Path, Slope};

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn generator(input: &str) -> TrailMap {
  TrailMap::from(input)
}

pub fn part1(map: &TrailMap) -> u32 {
  map.walk()
}

pub fn part2(map: &TrailMap) -> u32 {
  let g = map.make_graph();
  g.max_distance(&map.start, &map.end)
}

#[derive(Debug, Eq, PartialEq)]
enum Block {
  Path,
  Forest,
  Slope(i32, i32),
}

impl Block {
  fn from(input: char) -> Block {
    match input {
      '.' => Path,
      '#' => Forest,
      '^' => Slope(0, -1),
      'v' => Slope(0, 1),
      '<' => Slope(-1, 0),
      '>' => Slope(1, 0),
      _ => panic!("Unexpected input {input}"),
    }
  }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct XY {
  x: i32,
  y: i32,
}

#[derive(Clone)]
struct Hike {
  pos: XY,
  visited: HashSet<XY>,
}

impl Hike {
  fn mark_visited(&mut self) -> bool {
    if self.visited.contains(&self.pos) {
      false
    } else {
      self.visited.insert(self.pos.clone());
      true
    }
  }

  fn move_by(&mut self, dx: &i32, dy: &i32, tm: &TrailMap) -> bool {
    self.pos.x += *dx;
    self.pos.y += *dy;
    if self.pos.x < 0 || self.pos.x >= tm.width() as i32
      || self.pos.y < 0 || self.pos.y >= tm.height() as i32
      || tm.map[self.pos.y as usize][self.pos.x as usize] == Forest {
      return false;
    }

    let mark = self.mark_visited();
    if !mark {
      return mark;
    }

    match &tm.map[self.pos.y as usize][self.pos.x as usize] {
      Slope(dx, dy) => {
        self.move_by(dx, dy, tm)
      }
      _ => {
        true
      }
    }
  }

  fn len(&self) -> u32 {
    (self.visited.len() - 1) as u32
  }
}

pub struct TrailMap {
  map: Vec<Vec<Block>>,
  start: XY,
  end: XY,
}

impl TrailMap {
  fn from(input: &str) -> TrailMap {
    let map: Vec<Vec<Block>> = input.lines()
      .map(|l| l.chars().map(Block::from).collect())
      .collect();
    let start = XY { x: map[0].iter().position(|b| b == &Path).unwrap() as i32, y: 0 };
    let end = XY {
      x: map[map.len() - 1].iter().position(|b| b == &Path).unwrap() as i32,
      y: (map.len() - 1) as i32,
    };
    TrailMap { map, start, end }
  }

  fn height(&self) -> usize {
    self.map.len()
  }

  fn width(&self) -> usize {
    self.map[0].len()
  }

  fn single_next(&self, xy: &XY, visited: &HashSet<XY>) -> Option<XY> {
    let mut result: Option<XY> = None;

    for (x, y) in NEIGHBORS.iter() {
      let x = xy.x + x;
      let y = xy.y + y;
      if x > -1 && x < self.width() as i32
        && y > -1 && y < self.height() as i32
        && self.map[y as usize][x as usize] != Forest
        && !visited.contains(&XY{x, y}) {
        if result.is_none() {
          result = Some(XY{x, y});
        } else {
          result = None;
          break;
        }
      }
    }

    result
  }

  fn walk(&self) -> u32 {
    let mut stack: Vec<Hike> = Vec::new();
    let mut visited = HashSet::new();
    visited.insert(self.start.clone());
    stack.push(Hike {
      pos: self.start.clone(),
      visited,
    });
    let mut max_len = 0_u32;

    while !stack.is_empty() {
      let mut curr = stack.pop().unwrap();

      // If we have a single path, move forward and avoid cloning
      loop {
        match self.single_next(&curr.pos, &curr.visited) {
          None => break,
          Some(n) => {
            if !curr.move_by(&(n.x -curr.pos.x), &(n.y - curr.pos.y), self) {
              break;
            }
          }
        }
      }

      if curr.pos == self.end {
        if curr.len() > max_len {
          max_len = max_len.max(curr.len());
        }
        continue;
      }

      for (x, y) in NEIGHBORS.iter() {
        let mut next = curr.clone();
        if !next.move_by(x, y, self) { continue; }
        stack.push(next);
      }
    }
    max_len
  }

  fn make_graph(&self) -> Graph {
    let mut g = Graph::default();
    g.get_id(&self.start);

    let mut e = Edge {
      start: self.start.clone(),
      curr: self.start.clone(),
      visited: HashSet::from([self.start.clone()]),
    };
    // As this is a single trail block
    e.visit(e.nexts(self).pop().unwrap());

    let mut stack: Vec<Edge> = vec![e];

    while !stack.is_empty() {
      let mut e = stack.pop().unwrap();
      if g.is_vertex(&e.curr) {
        g.record_edge(&e.start, &e.curr, e.len());
        continue;
      }

      let mut nexts = e.nexts(self);
      let mut possibilities = nexts.len();
      match possibilities {
        0 => {
          // Complete edge
          g.record_edge(&e.start, &e.curr, e.len());
        }
        1 => {
          // Extend the edge
          while possibilities == 1 {
            e.visit(nexts.pop().unwrap());
            nexts = e.nexts(self);
            possibilities = nexts.len();
          }
          stack.push(e);
        }
        _ => {
          // As we have multiple paths, mark the vertex
          g.get_id(&e.curr);
          g.record_edge(&e.start, &e.curr, e.len());
          for next in nexts {
            // Create a new edge for each possibility
            let e = Edge {
              start: e.curr.clone(),
              curr: next.clone(),
              visited: HashSet::from([e.curr.clone(), next]),
            };
            stack.push(e);
          }
        }
      }
    }
    g
  }
}

#[derive(Default)]
struct Graph {
  vertices: HashMap<XY, usize>,
  edges: HashMap<(usize, usize), u32>,
}

impl Graph {
  fn get_id(&mut self, xy: &XY) -> usize {
    if !self.vertices.contains_key(xy) {
      self.vertices.insert(xy.clone(), self.vertices.len());
    }
    *self.vertices.get(xy).unwrap()
  }

  fn record_edge(&mut self, xy1: &XY, xy2: &XY, len: u32) {
    let id1 = self.get_id(xy1);
    let id2 = self.get_id(xy2);
    self.edges.entry((id1.min(id2), id2.max(id1)))
      .and_modify(|v| *v = (*v).max(len))
      .or_insert(len);
  }

  fn is_vertex(&self, xy: &XY) -> bool {
    self.vertices.contains_key(xy)
  }

  fn max_distance(&self, start: &XY, end: &XY) -> u32 {
    let mut paths: Vec<Vec<(usize, u32)>> = vec![vec![]; self.vertices.len()];
    for ((v1, v2), d) in &self.edges {
      paths[*v1].push((*v2, *d));
      paths[*v2].push((*v1, *d));
    }
    let start = self.vertices.get(start).unwrap();
    let end = self.vertices.get(end).unwrap();

    let mut max_dist = 0;
    let mut stack: Vec<(usize, u32, Vec<bool>)> = Vec::new();
    let mut visited = vec![false; self.vertices.len()];
    visited[*start] = true;
    stack.push((*start, 0, visited));

    while !stack.is_empty() {
      let (v, d, visited) = stack.pop().unwrap();
      if &v == end {
        max_dist = max_dist.max(d);
        continue;
      }
      for (n, d_to_n) in &paths[v] {
        if visited[*n] { continue; }
        let mut visited = visited.clone();
        visited[*n] = true;
        stack.push((*n, d + d_to_n, visited));
      }
    }
    max_dist
  }
}

struct Edge {
  start: XY,
  curr: XY,
  visited: HashSet<XY>,
}

impl Edge {
  fn len(&self) -> u32 {
    (self.visited.len() - 1) as u32
  }

  fn move_by(&self, dx: &i32, dy: &i32, tm: &TrailMap) -> Option<XY> {
    let mut next = self.curr.clone();
    next.x += *dx;
    next.y += *dy;
    if next.x < 0 || next.x >= tm.width() as i32
      || next.y < 0 || next.y >= tm.height() as i32
      || tm.map[next.y as usize][next.x as usize] == Forest
      || self.visited.contains(&next) {
      None
    } else {
      Some(next)
    }
  }

  fn visit(&mut self, xy: XY) {
    self.curr = xy;
    self.visited.insert(self.curr.clone());
  }

  fn nexts(&self, tm: &TrailMap) -> Vec<XY> {
    let nexts: Vec<XY> = NEIGHBORS.iter().filter_map(|(x, y)| {
      self.move_by(x, y, tm)
    }).collect();
    nexts
  }
}

#[cfg(test)]
mod tests {
  use crate::day23::{generator, part1, part2, XY};

  const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

  #[test]
  fn test_generator() {
    let tm = generator(INPUT);
    assert_eq!(23, tm.width());
    assert_eq!(XY { x: 1, y: 0 }, tm.start);
    assert_eq!(XY { x: 21, y: 22 }, tm.end);
    assert_eq!(23, tm.height());
  }

  #[test]
  fn test_part1() {
    let tm = generator(INPUT);
    assert_eq!(94, part1(&tm));
    //assert_eq!(94, tm.walk_max(false));
  }

  #[test]
  fn test_part2() {
    let tm = generator(INPUT);
    assert_eq!(154, part2(&tm));
    //assert_eq!(154, tm.walk_max(true));
  }

  const INPUT_SIMPLE: &str = "#.#######
#.#...###
#...#...#
#.#####.#
#.......#
######.##";

  #[test]
  fn test_graph() {
    let tm = generator(INPUT_SIMPLE);
    let g = tm.make_graph();
    assert_eq!(4, g.vertices.len());
    assert!(g.vertices.contains_key(&XY { x: 1, y: 0 }));
    assert!(g.vertices.contains_key(&XY { x: 6, y: 5 }));
    assert!(g.vertices.contains_key(&XY { x: 1, y: 2 }));
    assert!(g.vertices.contains_key(&XY { x: 6, y: 4 }));

    assert_eq!(g.edges.len(), 3);
    assert_eq!(g.edges.get(&(0, 1)).unwrap(), &2);
    assert_eq!(g.edges.get(&(1, 2)).unwrap(), &11);
    assert_eq!(g.edges.get(&(2, 3)).unwrap(), &1);

    assert_eq!(14, g.max_distance(&tm.start, &tm.end));
  }
}
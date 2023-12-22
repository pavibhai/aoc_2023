use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;


pub struct HeatLossMap {
  values: Vec<Vec<u8>>,
}

impl HeatLossMap {
  fn from(input: &str) -> HeatLossMap {
    let values = input.lines()
      .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
      .collect();
    HeatLossMap { values }
  }

  fn height(&self) -> u32 {
    self.values.len() as u32
  }

  fn width(&self) -> u32 {
    self.values[0].len() as u32
  }

  fn is_valid(&self, pos: &Path) -> bool {
    pos.x >= 0 && pos.x < self.width() as i32
      && pos.y >= 0 && pos.y < self.height() as i32
  }

  fn process_position(&self, p: &mut Path, hlv: &mut [Vec<[u32; 4]>], heap: &mut BinaryHeap<Reverse<Path>>,
                      prep_steps: usize, turn_steps: usize) {
    let dir_index = p.direction_index();
    // Perform the forward steps that cannot include a turn
    for _ in 0..prep_steps {
      if !self.forward(p) { return; }
    }

    // After each step we can turn so record them onto the heap
    for _ in 0..turn_steps {
      if !self.forward(p) { return; }
      if hlv[p.y as usize][p.x as usize][dir_index] > p.heat_loss {
        hlv[p.y as usize][p.x as usize][dir_index] = p.heat_loss;
        heap.push(Reverse(p.clone()));
      }
    }
  }

  fn forward(&self, p: &mut Path) -> bool {
    // Move forward and increment heat loss if the position is valid
    p.forward();
    if self.is_valid(p) {
      p.heat_loss += self.values[p.y as usize][p.x as usize] as u32;
      true
    } else {
      false
    }
  }

  fn compute_min_heat_loss(&self, prep_steps: usize, turn_steps: usize) -> Path {
    let mut hlv = vec![vec![[u32::MAX; 4]; self.width() as usize]; self.height() as usize];
    let mut heap = BinaryHeap::new();
    let mut start = Path::new();
    heap.push(Reverse(start.clone()));
    start.turn_left();
    heap.push(Reverse(start));

    while !heap.is_empty() {
      let mut p = heap.pop().unwrap().0;
      if p.x + 1 == self.width() as i32 && p.y + 1 == self.height() as i32 {
        return p;
      }

      // Turn left
      {
        let mut p = p.clone();
        p.turn_left();
        self.process_position(&mut p.clone(), &mut hlv, &mut heap, prep_steps, turn_steps);
      }


      // Turn right
      p.turn_right();
      self.process_position(&mut p, &mut hlv, &mut heap, prep_steps, turn_steps);
    }
    panic!("Could not find a path");
  }
}

#[derive(Default, Clone)]
struct Path {
  x: i32,
  y: i32,
  delta_x: i32,
  delta_y: i32,
  heat_loss: u32,
}

impl Path {
  fn new() -> Path {
    Path { x: 0, y: 0, delta_x: 1, delta_y: 0, heat_loss: 0 }
  }

  fn direction_index(&self) -> usize {
    match (self.delta_x, self.delta_y) {
      (-1, 0) => 0,
      (1, 0) => 1,
      (0, -1) => 2,
      (0, 1) => 3,
      _ => panic!("Unexpected direction")
    }
  }

  fn forward(&mut self) {
    self.x += self.delta_x;
    self.y += self.delta_y;
  }

  fn turn_left(&mut self) {
    match (self.delta_x, self.delta_y) {
      (1, 0) => {
        self.delta_x = 0;
        self.delta_y = -1;
      }
      (0, -1) => {
        self.delta_x = -1;
        self.delta_y = 0;
      }
      (-1, 0) => {
        self.delta_x = 0;
        self.delta_y = 1
      }
      (0, 1) => {
        self.delta_x = 1;
        self.delta_y = 0;
      }
      _ => panic!("Unexpected")
    }
  }

  fn turn_right(&mut self) {
    match (self.delta_x, self.delta_y) {
      (1, 0) => {
        self.delta_x = 0;
        self.delta_y = 1;
      }
      (0, 1) => {
        self.delta_x = -1;
        self.delta_y = 0;
      }
      (-1, 0) => {
        self.delta_x = 0;
        self.delta_y = -1;
      }
      (0, -1) => {
        self.delta_x = 1;
        self.delta_y = 0;
      }
      _ => panic!("Unexpected")
    }
  }
}

impl PartialEq for Path {
  fn eq(&self, other: &Self) -> bool {
    self.heat_loss.eq(&other.heat_loss)
  }
}

impl Eq for Path {}

impl PartialOrd for Path {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Path {
  fn cmp(&self, other: &Self) -> Ordering {
    self.heat_loss.cmp(&other.heat_loss)
  }
}

pub fn generator(input: &str) -> HeatLossMap {
  HeatLossMap::from(input)
}

pub fn part1(hlm: &HeatLossMap) -> u32 {
  let p = hlm.compute_min_heat_loss(0, 3);
  p.heat_loss
}

pub fn part2(hlm: &HeatLossMap) -> u32 {
  let p = hlm.compute_min_heat_loss(3, 7);
  p.heat_loss
}

#[cfg(test)]
mod tests {
  use crate::day17::{generator, HeatLossMap, part1, part2};

  fn input() -> String {
    "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".to_string()
  }

  #[test]
  fn test_generator() {
    let hlm = generator(&input());
    assert_eq!(hlm.height(), 13);
    assert_eq!(hlm.width(), 13);
    assert_eq!(hlm.values[0][0], 2);
    assert_eq!(hlm.values[12][12], 3);
  }

  #[test]
  fn test_compute_heat_loss() {
    let hlm = HeatLossMap::from("1229
1111");
    assert_eq!(4, hlm.compute_min_heat_loss(0, 3).heat_loss);

    let hlm = HeatLossMap::from("241343231
321545353");
    assert_eq!(32, part1(&hlm));
  }

  #[test]
  fn test_part1() {
    let hlm = generator(&input());
    assert_eq!(102, part1(&hlm));
  }

  #[test]
  fn test_part2() {
    let hlm = HeatLossMap::from("111111111111
999999999991
999999999991
999999999991
999999999991");
    assert_eq!(71, part2(&hlm));

    let hlm = generator(&input());
    assert_eq!(94, part2(&hlm));
  }
}
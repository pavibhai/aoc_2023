use std::collections::{HashMap};
use num::integer::lcm;

#[derive(Eq, PartialEq, Debug)]
struct Cycle {
  deltas: Vec<u32>,
  to_start: u32,
}

struct CycleItr {
  deltas: Vec<u32>,
  to_start: u32,
  curr_idx: usize,
  curr_value: u64,
}

impl CycleItr {
  fn from(cycle: &Cycle) -> CycleItr {
    CycleItr {
      deltas: cycle.deltas.clone(),
      to_start: cycle.to_start,
      curr_idx: 0,
      curr_value: 0,
    }
  }
}

impl Iterator for CycleItr {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    if self.curr_idx < self.deltas.len() {
      self.curr_value += self.deltas[self.curr_idx] as u64;
    } else {
      self.curr_value += self.to_start as u64;
    }
    self.curr_idx = (self.curr_idx + 1) % self.deltas.len();
    Some(self.curr_value)
  }
}

pub struct Document {
  instructions: Vec<char>,
  network: HashMap<String, (String, String)>,
}

impl Document {
  fn identify_cycle(&self, start: &String) -> Cycle {
    let mut instructs = self.instructions.iter()
      .enumerate().cycle();
    let mut deltas: Vec<u32> = Vec::new();
    let mut first: Option<(&String, usize)> = None;

    let mut curr = start;
    let mut steps = 0;
    loop {
      let (idx, instruct) = instructs.next().unwrap();
      steps += 1;

      let next = match (self.network.get(curr).unwrap(), instruct) {
        ((l, _), 'L') => {
          l
        }
        ((_, r), 'R') => {
          r
        }
        _ => panic!("Unexpected")
      };

      match next.ends_with('Z') {
        true if first.is_none() => {
          first = Some((curr, idx));
          deltas.push(steps);
        }
        true if first == Some((curr, idx)) => {
          return Cycle { to_start: steps - deltas.iter().sum::<u32>(), deltas };
        }
        true => {
          deltas.push(steps - deltas.last().unwrap());
        }
        _ => {}
      }
      curr = next;
    }
  }
}

pub fn generator(input: &str) -> Document {
  let (instructions, network) = input.split_once("\n\n").unwrap();
  let network = network.lines()
    .map(|line| {
      let (node, lr) = line.split_once(" = ").unwrap();
      let mut lr = lr.chars();
      lr.next();
      lr.next_back();
      let (l, r) = lr.as_str().split_once(", ").unwrap();
      (node.to_string(), (l.to_string(), r.to_string()))
    }).collect();

  Document {
    instructions: instructions.chars().collect(),
    network,
  }
}

pub fn part1(document: &Document) -> u32 {
  let mut curr = &"AAA".to_string();
  let mut steps = 0;
  let mut instructs = document.instructions.iter().cycle();
  while curr != "ZZZ" {
    steps += 1;
    match (document.network.get(curr).unwrap(), instructs.next().unwrap()) {
      ((l, _), 'L') => curr = l,
      ((_, r), 'R') => curr = r,
      _ => panic!("Unexpected")
    }
  }

  steps
}

pub fn part2(document: &Document) -> u64 {
  let cycles: Vec<Cycle> = document.network.keys()
    .filter_map(|c| {
      if c.ends_with('A') {
        Some(document.identify_cycle(c))
      } else {
        None
      }
    }).collect();

  if cycles.iter().all(|c| {
    (c.deltas.len() == 1 || c.deltas.iter().all(|o| o == c.deltas.first().unwrap()))
      && c.deltas.first().unwrap() == &c.to_start
  }) {
    cycles.iter().fold(1_u64, |a, c| {
      lcm(a, c.to_start as u64)
    })
  } else {
    let mut curr_max = 0_u64;
    let mut itrs: Vec<CycleItr> = cycles.iter().map(|c| {
      let mut i = CycleItr::from(c);
      i.next();
      curr_max = curr_max.max(i.curr_value);
      i
    }).collect();

    let mut moved = 1;
    while moved > 0 {
      moved = 0;
      for itr in itrs.iter_mut() {
        if itr.curr_value < curr_max {
          curr_max = curr_max.max(itr.next().unwrap());
          moved += 1;
        }
      }
    }
    curr_max
  }
}

#[cfg(test)]
mod tests {
  use crate::day8::{Cycle, CycleItr, generator, part1, part2};

  fn input() -> String {
    "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string()
  }

  #[test]
  fn test_generator() {
    let d = generator(&input());
    assert_eq!(2, d.instructions.len());
    assert_eq!(7, d.network.len());
  }

  #[test]
  fn test_part1() {
    let d = generator(&input());
    assert_eq!(2, part1(&d));

    let d = generator("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
    assert_eq!(6, part1(&d));
  }

  #[test]
  fn test_cycles() {
    let d = generator("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
    //assert_eq!(2, d.identify_repeat(&"11A".to_string()));
    assert_eq!(Cycle { deltas: vec![3, 3], to_start: 3 }, d.identify_cycle(&"22A".to_string()));
  }

  #[test]
  fn test_part2() {
    let d = generator("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
    assert_eq!(6, part2(&d));
  }

  #[test]
  fn test_cycle() {
    let c = Cycle { deltas: vec![19667], to_start: 19667 };
    let mut itr = CycleItr::from(&c).peekable();
    assert_eq!(itr.next().unwrap(), 19667);
    assert_eq!(itr.next().unwrap(), 19667 * 2);
    assert_eq!(itr.next().unwrap(), 19667 * 3);
  }
}
pub fn generator(input: &str) -> Snapshot {
  Snapshot::from(input)
}

pub fn part1(snapshot: &Snapshot) -> u32 {
  (snapshot.bricks.len() - snapshot.single_supporters().len()) as u32
}

pub fn part2(snapshot: &Snapshot) -> u32 {
  snapshot.single_supporters().iter().map(|id| snapshot.disintegrate(*id as usize))
    .sum()
}

#[derive(Debug, Clone)]
struct Brick {
  x1: u32,
  x2: u32,
  y1: u32,
  y2: u32,
  z1: u32,
  z2: u32,
}

impl Brick {
  fn from(input: &str) -> Brick {
    let (coord1, coord2) = input.split_once('~').unwrap();
    let splits1: Vec<u32> = coord1.split(',').map(|v| v.parse().unwrap())
      .collect();
    let splits2: Vec<u32> = coord2.split(',').map(|v| v.parse().unwrap())
      .collect();
    let x1 = splits1[0].min(splits2[0]);
    let x2 = splits1[0].max(splits2[0]);
    let y1 = splits1[1].min(splits2[1]);
    let y2 = splits1[1].max(splits2[1]);
    let z1 = splits1[2].min(splits2[2]);
    let z2 = splits1[2].max(splits2[2]);

    let mut equals = 0;
    if x1 == x2 { equals += 1; }
    if y1 == y2 { equals += 1; }
    if z1 == z2 { equals += 1; }
    assert!(equals > 1);

    Brick {
      x1,
      x2,
      y1,
      y2,
      z1,
      z2,
    }
  }

  fn fall(&mut self, id: usize, xy: &mut [Vec<u32>], brick_xy: &mut [Vec<i32>], rests_on: &mut [Vec<u32>]) -> bool {
    let mut fall_to = 0;
    let mut fell = false;
    for y in self.y1..=self.y2 {
      for x in self.x1..=self.x2 {
        fall_to = fall_to.max(xy[y as usize][x as usize]);
      }
    }
    let diff = self.z2 - self.z1;
    if self.z1 != fall_to + 1 {
      fell = true;
    }
    self.z1 = fall_to + 1;
    self.z2 = self.z1 + diff;
    for y in self.y1..=self.y2 {
      for x in self.x1..=self.x2 {
        if xy[y as usize][x as usize] == fall_to && brick_xy[y as usize][x as usize] > -1 {
          rests_on[id].push(brick_xy[y as usize][x as usize] as u32);
        }
        brick_xy[y as usize][x as usize] = id as i32;
        xy[y as usize][x as usize] = self.z2;
      }
    }
    rests_on[id].sort();
    rests_on[id].dedup();
    fell
  }
}

pub struct Snapshot {
  bricks: Vec<Brick>,
  rests_on: Vec<Vec<u32>>,
}

impl Snapshot {
  fn from(input: &str) -> Snapshot {
    let mut bricks: Vec<Brick> = input.lines()
      .map(Brick::from)
      .collect();

    let rests_on = Snapshot::fall(&mut bricks).0;
    Snapshot { bricks, rests_on }
  }

  fn fall(bricks: &mut Vec<Brick>) -> (Vec<Vec<u32>>, u32) {
    bricks.sort_by_key(|b| b.z1);
    let (max_x, max_y) = bricks.iter().fold((0, 0), |a, v| {
      (a.0.max(v.x2), a.1.max(v.y2))
    });
    let mut xy = vec![vec![0_u32; (max_x + 1) as usize]; (max_y + 1) as usize];
    let mut brick_xy = vec![vec![-1; (max_x + 1) as usize]; (max_y + 1) as usize];
    let mut rests_on = vec![vec![]; bricks.len()];
    let mut fallen_bricks = 0;
    for (id, brick) in bricks.iter_mut().enumerate() {
      if brick.fall(id, &mut xy, &mut brick_xy, &mut rests_on) {
        fallen_bricks += 1;
      }
    }

    (rests_on, fallen_bricks)
  }

  fn single_supporters(&self) -> Vec<u32> {
    let mut single_supporters = Vec::new();
    for on in &self.rests_on {
      if on.len() != 1 { continue; }
      single_supporters.push(on[0]);
    }
    single_supporters.sort();
    single_supporters.dedup();
    single_supporters
  }

  fn disintegrate(&self, id: usize) -> u32 {
    let mut falls: Vec<bool> = vec![false; self.bricks.len()];
    falls[id] = true;

    // Check other bricks to see which else might fall as a result of this
    for id in id+1..self.rests_on.len() {
      if self.rests_on[id].is_empty() { continue; }
      if self.rests_on[id].iter().all(|i| falls[*i as usize]) {
        falls[id] = true;
      }
    }
    (falls.iter().filter(|&f| *f).count() - 1) as u32
  }
}

#[cfg(test)]
mod tests {
  use crate::day22::{generator, part1, part2};

  const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

  #[test]
  fn test_generator() {
    let s = generator(INPUT);
    assert_eq!(7, s.bricks.len());
    assert_eq!(1, s.bricks[0].z1);
    assert_eq!(1, s.bricks[0].z2);
    assert_eq!(5, s.bricks[6].z1);
    assert_eq!(6, s.bricks[6].z2);
    assert!(s.rests_on[0].is_empty());
    assert_eq!(s.rests_on[1], vec![0]);
    assert_eq!(s.rests_on[2], vec![0]);
    assert_eq!(s.rests_on[3], vec![1, 2]);
    assert_eq!(s.rests_on[4], vec![1, 2]);
    assert_eq!(s.rests_on[5], vec![3, 4]);
    assert_eq!(s.rests_on[6], vec![5]);
  }

  #[test]
  fn test_part1() {
    let s = generator(INPUT);
    assert_eq!(5, part1(&s));
  }

  #[test]
  fn test_part2() {
    let s = generator(INPUT);
    assert_eq!(6, s.disintegrate(0));
    assert_eq!(1, s.disintegrate(5));
    assert_eq!(7, part2(&s));
  }
}
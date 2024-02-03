use std::collections::{HashMap, HashSet};
use prime_factorization::Factorization;
use crate::day24::Dimension::{X, Y, Z};

#[derive(Debug)]
enum Dimension {
  X,
  Y,
  Z,
}

impl Dimension {
  fn get_delta_fn(&self) -> fn(&Stone) -> i64 {
    match self {
      X => Stone::get_dx,
      Y => Stone::get_dy,
      Z => Stone::get_dz,
    }
  }

  fn get_dim_fn(&self) -> fn(&Stone) -> i64 {
    match self {
      X => Stone::get_x,
      Y => Stone::get_y,
      Z => Stone::get_z,
    }
  }
}

#[derive(Debug)]
struct Possibility {
  tgt: usize,
  src: usize,
  delta: Vec<(i64, i64)>,
}

impl Possibility {
  fn tgt_stone<'a>(&'a self, hail: &'a Hail) -> &Stone {
    &hail.stones[self.tgt]
  }

  fn src_stone<'a>(&'a self, hail: &'a Hail) -> &Stone {
    &hail.stones[self.src]
  }
}

pub struct Hail {
  stones: Vec<Stone>,
}

impl Hail {
  fn from(input: &str) -> Hail {
    let stones = input.lines().map(Stone::from)
      .collect();
    Hail { stones }
  }

  fn count_crossings(&self, min_d: u64, max_d: u64) -> u32 {
    let min_d = min_d as f64;
    let max_d = max_d as f64;
    let mut crossings = 0;
    for s1_idx in 0..self.stones.len() - 1 {
      for s2_idx in s1_idx + 1..self.stones.len() {
        match self.stones[s1_idx].cross_xy(&self.stones[s2_idx]) {
          Some((x, y)) if x >= min_d && y >= min_d && x <= max_d && y <= max_d => {
            crossings += 1;
          }
          _ => {}
        }
      }
    }

    crossings
  }

  fn find_unique_by(&self, key: fn(&Stone) -> i64) -> Vec<(usize, usize)> {
    let mut map: HashMap<i64, Vec<usize>> = HashMap::new();
    for (i, s) in self.stones.iter().enumerate() {
      let v = key(s);
      map.entry(v)
        .and_modify(|v| v.push(i))
        .or_insert(vec![i]);
    }
    map.iter().filter_map(|(_, v)|
      if v.len() > 1 {
        let mut combinations = Vec::new();
        for i in 0..v.len() - 1 {
          for j in i + 1..v.len() {
            combinations.push((v[i], v[j]))
          }
        }
        Some(combinations)
      } else {
        None
      })
      .flatten()
      .collect()
  }

  fn compute_min_possibilities(&self, dim: Dimension) -> Possibility {
    let get_delta = dim.get_delta_fn();
    let get_dim = dim.get_dim_fn();

    let d_equals = self.find_unique_by(get_delta);
    let mut idx: usize = 0;
    let mut min_possibilities = vec![];
    for (id, entry) in d_equals.iter().enumerate() {
      let s1 = &self.stones[entry.0];
      let s2 = &self.stones[entry.1];
      let possible_time = Factorization::run(s1.time_from(s2).unsigned_abs());
      if id == 0 {
        min_possibilities = possible_time.factors;
      } else if min_possibilities.len() > possible_time.factors.len() {
        min_possibilities = possible_time.factors;
        idx = id;
      }
    }

    let delta = all_factors(&min_possibilities).iter()
      .flat_map(|t| {
        let s1 = &self.stones[d_equals[idx].0];
        let s2 = &self.stones[d_equals[idx].1];
        [(get_delta(s1) + ((get_dim(s1) - get_dim(s2)) / *t as i64), *t as i64),
          (get_delta(s1) - ((get_dim(s1) - get_dim(s2)) / *t as i64), -(*t as i64))]
      })
      .collect();

    Possibility {
      tgt: d_equals[idx].0,
      src: d_equals[idx].1,
      delta,
    }
  }

  fn compute_dx_dy_dz(&self) -> (Possibility, Possibility, Possibility) {
    println!("For dx");
    let dx_possibilities = self.compute_min_possibilities(X);
    println!("{:?}", dx_possibilities);
    println!("For dy");
    let dy_possibilities = self.compute_min_possibilities(Y);
    println!("{:?}", dy_possibilities);
    println!("For dz");
    let dz_possibilities = self.compute_min_possibilities(Z);
    println!("{:?}", dz_possibilities);
    (dx_possibilities, dy_possibilities, dz_possibilities)
  }

  fn find_hit_all_stone(&self) -> Stone {
    let (dxp, dyp, dzp) = self.compute_dx_dy_dz();
    let mut possibilities = HashSet::new();
    let x_tgt = dxp.tgt_stone(self);
    let x_src = dxp.src_stone(self);
    let y_tgt = dyp.tgt_stone(self);
    let y_src = dyp.src_stone(self);
    let z_tgt = dzp.tgt_stone(self);
    let z_src = dzp.src_stone(self);
    for (dx, tx) in &dxp.delta {
      for (dy, ty) in &dyp.delta {
        let t_y1 = (y_tgt.x - y_src.x + (y_tgt.dx * ty) - (dx * ty)) / (y_src.dx - y_tgt.dx);
        if t_y1 <= 0 {
          // Invalid combination
          continue;
        }
        for (dz, tz) in &dzp.delta {
          let t_z1 = (z_tgt.x - z_src.x + (z_tgt.dx * tz) - (dx * tz)) / (z_src.dx - z_tgt.dx);
          if t_z1 <= 0 { continue; }
          let t_z2 = (z_tgt.y - z_src.y + (z_tgt.dy * tz) - (dy * tz)) / (z_src.dy - z_tgt.dy);
          if t_z2 != t_z1 { continue; }

          let t_y2 = (y_tgt.z - y_src.z + (y_tgt.dz * ty) - (dz * ty)) / (y_src.dz - y_tgt.dz);
          if t_y2 != t_y1 { continue; }

          // Check for x possibilities
          let t_x1 = (x_tgt.y - x_src.y + (x_tgt.dy * tx) - (dy * tx)) / (x_src.dy - x_tgt.dy);
          if t_x1 <= 0 { continue; }
          let t_x2 = (x_tgt.z - x_src.z + (x_tgt.dz * tx) - (dz * tx)) / (x_src.dz - x_tgt.dz);
          if t_x2 != t_x1 { continue; }

          let x = z_src.x + z_src.dx * t_z1 - dx * t_z1;
          let y = z_src.y + z_src.dy * t_z1 - dy * t_z1;
          let z = z_src.z + z_src.dz * t_z1 - dz * t_z1;
          possibilities.insert(Stone { x, y, z, dx: *dx, dy: *dy, dz: *dz });
        }
      }
    }
    println!("Possibilities: {:?}", possibilities);
    assert_eq!(1, possibilities.len());
    let s = possibilities.drain().next().unwrap();
    s
  }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Stone {
  x: i64,
  y: i64,
  z: i64,
  dx: i64,
  dy: i64,
  dz: i64,
}

fn all_factors(prime_factors: &[u64]) -> Vec<u64> {
  let mut positive_factors = Vec::new();
  for v in 0..2_u32.pow(prime_factors.len() as u32) {
    let mut factor = 1;
    for (i, f) in prime_factors.iter().enumerate() {
      if (1 << i & v) > 0 {
        factor *= f;
      }
    }
    positive_factors.push(factor);
  }
  positive_factors
}

impl Stone {
  fn get_x(&self) -> i64 {
    self.x
  }

  fn get_y(&self) -> i64 {
    self.y
  }

  fn get_z(&self) -> i64 {
    self.z
  }

  fn get_dx(&self) -> i64 {
    self.dx
  }

  fn get_dy(&self) -> i64 {
    self.dy
  }

  fn get_dz(&self) -> i64 {
    self.dz
  }

  fn from(input: &str) -> Stone {
    let (pos, velocity) = input.split_once(" @ ").unwrap();
    let mut pos = pos.split(", ");
    let mut velocity = velocity.split(", ");
    let x: i64 = pos.next().unwrap().trim().parse().unwrap();
    let y: i64 = pos.next().unwrap().trim().parse().unwrap();
    let z: i64 = pos.next().unwrap().trim().parse().unwrap();
    let dx: i64 = velocity.next().unwrap().trim().parse().unwrap();
    let dy: i64 = velocity.next().unwrap().trim().parse().unwrap();
    let dz: i64 = velocity.next().unwrap().trim().parse().unwrap();
    Stone { x, y, z, dx, dy, dz }
  }

  fn at_time(&self, t: f64) -> (f64, f64, f64) {
    (
      self.x as f64 + self.dx as f64 * t,
      self.y as f64 + self.dy as f64 * t,
      self.z as f64 + self.dz as f64 * t
    )
  }

  fn cross_xy(&self, other: &Stone) -> Option<(f64, f64)> {
    if other.dy * self.dx - other.dx * self.dy == 0 {
      return None;
    }

    let other_t = (((other.x - self.x) * self.dy) - ((other.y - self.y) * self.dx)) as f64
      / (other.dy * self.dx - other.dx * self.dy) as f64;
    let self_t = (other.x - self.x) as f64 / self.dx as f64
      + other.dx as f64 * other_t / self.dx as f64;

    if other_t < 0f64 || self_t < 0f64 {
      None
    } else {
      let (x, y, _) = other.at_time(other_t);
      Some((x, y))
    }
  }

  fn time_from(&self, stone: &Stone) -> i64 {
    if self.dx == stone.dx {
      self.x - stone.x
    } else if self.dy == stone.dy {
      self.y - stone.y
    } else if self.dz == stone.dz {
      self.z - stone.z
    } else {
      panic!("Cannot estimate time from stone {:?} to {:?}", stone, self);
    }
  }
}

pub fn generator(input: &str) -> Hail {
  Hail::from(input)
}

pub fn part1(hail: &Hail) -> i64 {
  hail.count_crossings(200000000000000, 400000000000000) as i64
}

pub fn part2(hail: &Hail) -> i64 {
  let s = hail.find_hit_all_stone();
  s.x + s.y + s.z
}

#[cfg(test)]
mod tests {
  use crate::day24::{all_factors, Hail, part2, Stone};

  const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

  #[test]
  fn test_generator() {
    let h = Hail::from(INPUT);
    assert_eq!(5, h.stones.len());
  }

  #[test]
  fn test_stone() {
    Stone::from("297310270744292, 292515986537934, 398367816281800 @ -130, 46, -342");
  }

  #[test]
  fn test_meets() {
    let s1 = Stone::from("19, 13, 30 @ -2, 1, -2");
    let s2 = Stone::from("18, 19, 22 @ -1, -1, -2");
    let (x, y) = s1.cross_xy(&s2).unwrap();
    assert_eq!(x as i32, 14);
    assert_eq!(y as i32, 15);
    assert!(x > 14.333);
    assert!(y > 15.333);

    let s1 = Stone::from("19, 13, 30 @ -2, 1, -2");
    let s2 = Stone::from("20, 25, 34 @ -2, -2, -4");
    let (x, y) = s1.cross_xy(&s2).unwrap();
    assert_eq!(x as i32, 11);
    assert_eq!(y as i32, 16);
    assert!(x > 11.6);
    assert!(y > 16.6);

    let s1 = Stone::from("19, 13, 30 @ -2, 1, -2");
    let s2 = Stone::from("20, 19, 15 @ 1, -5, -3");
    assert_eq!(None, s1.cross_xy(&s2));

    let s1 = Stone::from("18, 19, 22 @ -1, -1, -2");
    let s2 = Stone::from("20, 25, 34 @ -2, -2, -4");
    assert_eq!(None, s1.cross_xy(&s2));
  }

  #[test]
  fn test_part1() {
    let h = Hail::from(INPUT);
    assert_eq!(2, h.count_crossings(7, 27));
  }

  #[test]
  fn test_part2() {
    let h = Hail::from(INPUT);
    assert_eq!(part2(&h), 47);
  }

  #[test]
  fn test_factors() {
    let factors = vec![2, 197, 262590870317];
    let all_positive_divisors = all_factors(&factors);
    assert_eq!(all_positive_divisors, vec![1, 2, 197, 394, 262590870317,
                                           262590870317 * 2,
                                           262590870317 * 197,
                                           factors.iter().product()]);
  }
}
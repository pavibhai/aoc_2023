pub struct Race {
  time: u64,
  dist: u64,
}

impl Race {
  fn min_dist(&self) -> u64 {
    self.dist + 1
  }

  fn range(&self) -> (u64, u64) {
    let sqrt = (((self.time * self.time) - (4 * self.min_dist())) as f64).sqrt();
    let low = ((self.time as f64 - sqrt) / 2_f64).ceil() as u64;
    let high = ((self.time as f64 + sqrt) / 2_f64).floor() as u64;
    (low, high)
  }
}

pub fn line_split_whitespace(input: &str) -> Vec<u64> {
  let (_, v) = input.split_once(':').unwrap();
  v.split_whitespace()
    .filter_map(|e| if e.is_empty() { None } else { Some(e.parse().unwrap()) }).collect()
}

pub fn generator(input: &str) -> Vec<Race> {
  let mut lines = input.lines();
  let times: Vec<u64> = line_split_whitespace(lines.next().unwrap());
  let distances: Vec<u64> = line_split_whitespace(lines.next().unwrap());
  assert_eq!(times.len(), distances.len());

  times.iter().zip(distances).map(|(&t, d)| {
    Race {
      time: t,
      dist: d,
    }
  }).collect()
}

pub fn part1(races: &[Race]) -> u64 {
  races.iter().map(|r| {
    let (l, h) = r.range();
    h - l + 1
  }).product()
}

pub fn part2(races: &[Race]) -> u64 {
  let mut dist = 0_u64;
  let mut time = 0_u64;
  for race in races {
    dist *= 10_u64.pow(race.dist.checked_ilog10().unwrap_or(0) + 1);
    dist += race.dist;
    time *= 10_u64.pow(race.time.checked_ilog10().unwrap_or(0) + 1);
    time += race.time;
  }
  let race = Race { time, dist };
  let (l, h) = race.range();
  h - l + 1
}

#[cfg(test)]
mod tests {
  use crate::day6::{generator, part1, part2, Race};

  fn input() -> String {
    "Time:      7  15   30
Distance:  9  40  200".to_string()
  }

  #[test]
  fn test_generator() {
    let races = generator(&input());
    assert_eq!(3, races.len());
    assert_eq!(races[0].time, 7);
    assert_eq!(races[0].dist, 9);

    assert_eq!(races[2].time, 30);
    assert_eq!(races[2].dist, 200);
  }

  #[test]
  fn test_range() {
    assert_eq!((2, 5), Race { time: 7, dist: 9 }.range());
    assert_eq!((4, 11), Race { time: 15, dist: 40 }.range());
    assert_eq!((11, 19), Race { time: 30, dist: 200 }.range());
  }

  #[test]
  fn test_part1() {
    let races = generator(&input());
    assert_eq!(288, part1(&races));
  }

  #[test]
  fn test_part2() {
    let races = generator(&input());
    assert_eq!(71503, part2(&races));
  }
}
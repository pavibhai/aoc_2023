const EMPTY_SPACE: char = '.';

#[derive(Eq, PartialEq, Debug)]
struct XY {
  x: usize,
  y: usize,
}

pub struct Image {
  galaxies: Vec<XY>,
  empty_rows: Vec<bool>,
  empty_cols: Vec<bool>,
}

fn distance_between(start: usize, end: usize, empty_space: &[bool]) -> (u64, u64) {
  if start < end {
    empty_space[start+1..end].iter().fold((1, 0), |(n, e), v| {
      if *v {(n, e +1)} else {(n+1, e)}
    })
  } else {
    (0, 0)
  }

}

impl Image {
  fn _compute_total_distances_w_expansion(&self, empty_as: u64) -> u64 {
    let (n, e) = self.compute_total_distances();
    n + (e * empty_as)
  }
  fn compute_total_distances(&self) -> (u64, u64) {
    let mut normal_moves = 0;
    let mut empty_moves = 0;

    for i in 0..self.galaxies.len() - 1 {
      for j in i + 1..self.galaxies.len() {
        let (n, e) = self.distance_between(&self.galaxies[i], &self.galaxies[j]);
        normal_moves += n;
        empty_moves += e;
      }
    }
    (normal_moves, empty_moves)
  }
  fn distance_between(&self, g1: &XY, g2: &XY) -> (u64, u64) {
    let (normal_moves, empty_moves) = if g1.x < g2.x {
      distance_between(g1.x, g2.x, &self.empty_cols)
    } else {
      distance_between(g2.x, g1.x, &self.empty_cols)
    };
    let (n, e) = if g1.y < g2. y {
      distance_between(g1.y, g2.y, &self.empty_rows)
    } else {
      distance_between(g2.y, g1.y, &self.empty_rows)
    };

    (normal_moves + n, empty_moves + e)
  }

  fn _distance_between_w_expansion(&self, g1: &XY, g2: &XY, empty_as: u64) -> u64 {
    let (n, e) = self.distance_between(g1, g2);
    n + (e * empty_as)
  }

  fn from(input: &str) -> Image {
    let chars: Vec<Vec<char>> = input.lines()
      .map(|line| line.chars().collect())
      .collect();

    let empty_rows: Vec<bool> = chars.iter()
      .map(|line| {
        line.iter().all(|c| c == &EMPTY_SPACE)
      }).collect();

    let empty_cols: Vec<bool> = (0..chars.first().unwrap().len()).map(|c| {
      chars.iter().all(|row| row[c] == EMPTY_SPACE)
    }).collect();
    let mut galaxies = Vec::new();
    chars.iter().enumerate().for_each(|(y, r)| {
      r.iter().enumerate().for_each(|(x, c)| {
        if c != &EMPTY_SPACE {
          galaxies.push(XY { x, y });
        }
      })
    });

    Image {
      galaxies,
      empty_rows,
      empty_cols,
    }
  }
}

pub fn generator(input: &str) -> (u64, u64) {
  Image::from(input).compute_total_distances()
}

pub fn part1(distances: &(u64, u64)) -> u64 {
  let (n, e) = distances;
  n + (e * 2)
}

pub fn part2(distances: &(u64, u64)) -> u64 {
  let (n, e) = distances;
  n + (e * 1000000)
}

#[cfg(test)]
mod tests {
  use crate::day11::{generator, Image, part1, XY};

  fn input() -> String {
    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string()
  }

  #[test]
  fn test_generator() {
    let i = Image::from(&input());
    assert_eq!(9, i.galaxies.len());
    assert_eq!(XY { x: 3, y: 0 }, i.galaxies[0]);
    assert_eq!(XY { x: 4, y: 9 }, i.galaxies[8]);
    assert!(!i.empty_rows[0]);
    assert!(i.empty_rows[3]);
    assert!(i.empty_rows[7]);
    assert!(i.empty_cols[2]);
    assert!(i.empty_cols[5]);
    assert!(i.empty_cols[8]);
  }

  #[test]
  fn test_moves() {
    let i = Image::from(&input());
    assert_eq!(9, i._distance_between_w_expansion(&i.galaxies[4], &i.galaxies[8], 2));
    assert_eq!(15, i._distance_between_w_expansion(&i.galaxies[0], &i.galaxies[6], 2));
    assert_eq!(5, i._distance_between_w_expansion(&i.galaxies[7], &i.galaxies[8], 2));
  }

  #[test]
  fn test_part1() {
    let i = generator(&input());
    assert_eq!(374, part1(&i));
  }

  #[test]
  fn test_part2() {
    let i = Image::from(&input());
    assert_eq!(1030, i._compute_total_distances_w_expansion(10));
    assert_eq!(8410, i._compute_total_distances_w_expansion(100));
  }
}
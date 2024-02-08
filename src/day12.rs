use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::day12::Status::{DAMAGED, OPERATIONAL, UNKNOWN};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Status {
  UNKNOWN,
  OPERATIONAL,
  DAMAGED,
}

impl Status {
  fn to_char(&self) -> char {
    match self {
      UNKNOWN => { '?' }
      OPERATIONAL => { '.' }
      DAMAGED => { '#' }
    }
  }
}

#[derive(Debug)]
pub struct PumpRow {
  pumps: Vec<Status>,
  groups: Vec<usize>,
  last_damaged: Option<usize>,
}

impl Display for PumpRow {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    for p in &self.pumps {
      output.push(p.to_char());
    }
    output.push(' ');
    for g in &self.groups {
      output.push_str(&g.to_string());
      output.push(',');
    }
    write!(f, "{output}")
  }
}

impl PumpRow {
  fn from(input: &str) -> PumpRow {
    let (pumps, groups) = input.split_once(' ').unwrap();
    let pumps: Vec<Status> = pumps.chars().map(|c| match c {
      '#' => DAMAGED,
      '.' => OPERATIONAL,
      '?' => UNKNOWN,
      _ => panic!("Unexpected character for pump status {c}"),
    }).collect();
    let groups = groups.split(',').map(|v| v.parse().unwrap()).collect();
    let last_damaged = pumps.iter().rposition(|s| s == &DAMAGED);

    PumpRow {
      pumps,
      groups,
      last_damaged,
    }
  }

  fn group_starts(&self, group_idx: usize, start: usize) -> Vec<usize> {
    let mut starts = Vec::new();

    for idx in start..self.pumps.len() {
      match idx + self.groups[group_idx] {
        u if u > self.pumps.len() => {
          continue;
        }
        u if u < self.pumps.len() && &self.pumps[u] == &DAMAGED => {
          continue;
        }
        u if self.pumps[idx..u].iter().any(|s| s == &OPERATIONAL) => {
          continue;
        }
        _ if idx > 0 && &self.pumps[idx - 1] == &DAMAGED => {
          continue;
        }
        _ => starts.push(idx),
      }
    }

    starts
  }

  fn count_possibilities(&self) -> u64 {
    let group_starts = self.compute_group_starts();
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
    count_possibilities_w_cache(&self, &group_starts, 0, 0, &mut cache)
  }

  fn compute_group_starts(&self) -> Vec<Vec<usize>> {
    let mut group_starts: Vec<Vec<usize>> = Vec::new();
    for i in 0..self.groups.len() {
      let start = if i > 0 { group_starts[i - 1][0] + self.groups[i - 1] + 1 } else { 0 };
      group_starts.push(self.group_starts(i, start));
    }
    group_starts
  }

  fn unfold(&self) -> PumpRow {
    let mut pumps = self.pumps.clone();
    let mut groups = self.groups.clone();

    for _ in 0..4 {
      pumps.push(UNKNOWN);
      pumps.extend(&self.pumps);

      groups.extend(&self.groups);
    }
    let last_damaged = pumps.iter().rposition(|s| s == &DAMAGED);

    PumpRow {
      pumps,
      groups,
      last_damaged,
    }
  }
}


fn count_possibilities_w_cache(pr: &PumpRow, group_starts: &Vec<Vec<usize>>, level: usize, min_value: usize,
                               cache: &mut HashMap<(usize, usize), u64>) -> u64 {
  if let Some(sum) = cache.get(&(level, min_value)) {
    return *sum;
  } else {
    let sum = count_possibilities(pr, group_starts, level, min_value, cache);
    cache.insert((level, min_value), sum);
    sum
  }
}

fn count_possibilities(pr: &PumpRow, group_starts: &Vec<Vec<usize>>, level: usize, min_value: usize,
                       cache: &mut HashMap<(usize, usize), u64>) -> u64 {
  let start = group_starts[level].binary_search(&min_value).unwrap_or_else(|i| i);

  if start >= group_starts[level].len() {
    return 0;
  };

  if level + 1 == group_starts.len() {
    let mut sum = 0;
    // compute last damaged position
    for i in start..group_starts[level].len() {
      let mv = &group_starts[level][i];
      // If we left a damaged pipe then the reset of the options are not valid.
      if pr.pumps[min_value..*mv].iter().any(|s| s == &DAMAGED) { break; }
      // If we have a trailing damaged that is not part of this then this is not a valid choice
      if pr.last_damaged.is_some_and(|v| v > *mv + pr.groups[level]) { continue; }
      if pr.last_damaged.unwrap_or(*mv) < *mv + pr.groups[level] {
        sum += 1;
      }
    }
    sum
  } else {
    let mut sum = 0;
    for i in start..group_starts[level].len() {
      let mv = &group_starts[level][i];
      // If we left a damaged pipe then the reset of the options are not valid.
      if pr.pumps[min_value..*mv].iter().any(|s| s == &DAMAGED) { break; }
      sum += count_possibilities_w_cache(pr, group_starts, level + 1, mv + 1 + pr.groups[level], cache);
    }
    sum
  }
}

pub fn generator(input: &str) -> Vec<PumpRow> {
  input.lines()
    .map(|line| PumpRow::from(line))
    .collect()
}

pub fn part1(rows: &[PumpRow]) -> u64 {
  rows.iter().map(|pr| {
    let p = pr.count_possibilities();
    p
  }).sum()
}

pub fn part2(rows: &[PumpRow]) -> u64 {
  rows.iter().map(|pr| pr.unfold().count_possibilities()).sum()
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2, PumpRow};
  use crate::day12::Status::{DAMAGED, OPERATIONAL, UNKNOWN};

  fn input() -> String {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string()
  }

  #[test]
  fn test_generator() {
    let rows = generator(&input());
    assert_eq!(6, rows.len());
  }

  #[test]
  fn test_pump_row() {
    let pr = PumpRow::from("???.### 1,1,3");
    assert_eq!(&pr.pumps, &vec![UNKNOWN, UNKNOWN, UNKNOWN, OPERATIONAL, DAMAGED, DAMAGED, DAMAGED]);
    assert_eq!(&pr.groups, &vec![1, 1, 3]);
    let group_starts = pr.compute_group_starts();
    assert_eq!(group_starts[0], vec![0, 1, 2]);
    assert_eq!(group_starts[1], vec![2]);
    assert_eq!(group_starts[2], vec![4]);

    let pr = PumpRow::from(".??..??...?##. 1,1,3");
    let group_starts = pr.compute_group_starts();
    assert_eq!(group_starts[0], vec![1, 2, 5, 6]);
    assert_eq!(group_starts[1], vec![5, 6]);
    assert_eq!(group_starts[2], vec![10]);

    let pr = PumpRow::from("?#?#?#?#?#?#?#? 1,3,1,6");
    let group_starts = pr.compute_group_starts();
    assert_eq!(group_starts[0], vec![1, 3, 5, 7, 9, 11, 13]);
    assert_eq!(group_starts[1], vec![3, 5, 7, 9, 11]);
    assert_eq!(group_starts[2], vec![7, 9, 11, 13]);
    assert_eq!(group_starts[3], vec![9]);

    let pr = PumpRow::from("????.######..#####. 1,6,5");
    let group_starts = pr.compute_group_starts();
    assert_eq!(group_starts[0], vec![0, 1, 2, 3]);
    assert_eq!(group_starts[1], vec![5]);
    assert_eq!(group_starts[2], vec![13]);
  }

  #[test]
  fn test_possibilities() {
    let pr = PumpRow::from("???.### 1,1,3");
    assert_eq!(1, pr.count_possibilities());
    assert_eq!(1, pr.unfold().count_possibilities());

    let pr = PumpRow::from(".??..??...?##. 1,1,3");
    assert_eq!(4, pr.count_possibilities());
    assert_eq!(16384, pr.unfold().count_possibilities());

    let pr = PumpRow::from("?#?#?#?#?#?#?#? 1,3,1,6");
    assert_eq!(1, pr.count_possibilities());
    assert_eq!(1, pr.unfold().count_possibilities());

    let pr = PumpRow::from("????.#...#... 4,1,1");
    assert_eq!(1, pr.count_possibilities());
    assert_eq!(16, pr.unfold().count_possibilities());

    let pr = PumpRow::from("????.######..#####. 1,6,5");
    assert_eq!(4, pr.count_possibilities());
    assert_eq!(2500, pr.unfold().count_possibilities());

    let pr = PumpRow::from("?###???????? 3,2,1");
    assert_eq!(10, pr.count_possibilities());
    //assert_eq!(506250, pr.unfold().count_possibilities());
  }

  #[test]
  fn test_failures() {
    let pr = PumpRow::from("###???#??#??????? 4,4,1");
    assert_eq!(6, pr.count_possibilities());

    let pr = PumpRow::from("##.?.????#??.?#?## 2,1,1,1,1,5");
    assert_eq!(5, pr.count_possibilities());

    let pr = PumpRow::from("##.???#??.??..# 2,1,1,1,1");
    assert_eq!(8, pr.count_possibilities());
  }

  #[test]
  fn test_part1() {
    let rows = generator(&input());
    assert_eq!(21, part1(&rows));
  }

  #[test]
  fn test_part2() {
    let rows = generator(&input());
    assert_eq!(525152, part2(&rows));
  }
}
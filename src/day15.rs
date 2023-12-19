use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<Vec<char>> {
  input.lines().next().unwrap()
    .split(',')
    .map(|line| line.chars().collect())
    .collect()
}

pub fn part1(codes: &[Vec<char>]) -> u32 {
  codes.iter().map(|v| chars_to_hash(v))
    .sum()
}

fn chars_to_hash(chars: &[char]) -> u32 {
  chars.iter().fold(0_u32, |a, c| ((a + *c as u32) * 17_u32) % 256_u32)
}

pub fn part2(codes: &[Vec<char>]) -> u32 {
  let mut boxes: Vec<HashMap<&[char], (u8, usize)>> = vec![HashMap::new(); 256];
  for (r, code) in codes.iter().enumerate() {
    let label = if code.last().unwrap() == &'-' {
      &code[..code.len() - 1]
    } else {
      &code[..code.len() - 2]
    };
    let box_id = chars_to_hash(label);
    match code.last() {
      Some('-') => {
        boxes[box_id as usize].remove_entry(&code[..code.len() - 1]);
      }
      _ => {
        let fl = code.last().unwrap().to_digit(10).unwrap() as u8;
        boxes[box_id as usize].entry(&code[..code.len() - 2])
          .and_modify(|(v, _)| {
            *v = fl;
          }).or_insert((fl, r));
      }
    }
  }

  // Compute scores
  let mut focus_power = 0_u32;
  for (r, b) in boxes.iter().enumerate() {
    let mut values: Vec<&(u8, usize)> = b.values().collect();
    values.sort_by_key(|(_, idx)| idx);
    focus_power = values.iter().enumerate().fold(focus_power, |fp, (slot, (fl, _))| {
      fp + ((r + 1) as u32 * (slot + 1) as u32 * *fl as u32)
    });
  }

  focus_power
}

#[cfg(test)]
mod tests {
  use crate::day15::{chars_to_hash, generator, part1, part2};

  #[test]
  fn test_ascii_codes() {
    assert_eq!(65, 'A' as u32);
    let chars: Vec<char> = "HASH".chars().collect();
    assert_eq!(52, chars_to_hash(&chars));
    let chars: Vec<char> = "rn=1".chars().collect();
    assert_eq!(30, chars_to_hash(&chars));
    let chars: Vec<char> = "cm-".chars().collect();
    assert_eq!(253, chars_to_hash(&chars));
    let chars: Vec<char> = "qp-".chars().collect();
    assert_eq!(14, chars_to_hash(&chars));
  }

  fn input() -> String {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
  }

  #[test]
  fn test_part1() {
    let codes = generator(&input());
    assert_eq!(1320, part1(&codes));
  }

  #[test]
  fn test_part2() {
    let codes = generator(&input());
    assert_eq!(145, part2(&codes));
  }
}

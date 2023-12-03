use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct XY {
  x: usize,
  y: usize,
}

impl XY {
  fn create(x: usize, y: usize) -> XY {
    XY {
      x,
      y,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Number {
  start: XY,
  chars: usize,
  value: u32,
}

impl Number {
  fn empty() -> Number {
    Number::create(XY::create(0, 0))
  }
  fn create(start: XY) -> Number {
    Number {
      start,
      chars: 0,
      value: 0,
    }
  }

  fn clear(&mut self) {
    self.value = 0;
    self.chars = 0;
  }

  fn add_digit(&mut self, digit: u32) {
    self.value *= 10;
    self.value += digit;
    self.chars += 1;
  }

  fn is_empty(&self) -> bool {
    self.chars == 0
  }

  fn set_xy(&mut self, x: usize, y: usize) {
    self.start.x = x;
    self.start.y = y;
  }
}

pub struct Schematic {
  numbers: Vec<Number>,
  symbols: HashMap<XY, char>,
}

pub fn part1(schematic: &Schematic) -> u32 {
  schematic.numbers.iter().filter_map(|number| {
    for x in [number.start.x as i32 - 1, (number.start.x + number.chars) as i32] {
      if x >= 0 && schematic.symbols.contains_key(&XY::create(x as usize, number.start.y)) {
        return Some(number.value);
      }
    }
    for y in [number.start.y as i32 - 1, number.start.y as i32 + 1] {
      if y >= 0 {
        for x in (number.start.x as i32 - 1)..=(number.start.x + number.chars) as i32 {
          if x >= 0 && schematic.symbols.contains_key(&XY::create(x as usize, y as usize)) {
            return Some(number.value);
          }
        }
      }
    }
    None
  }).sum()
}

pub fn part2(schematic: &Schematic) -> u32 {
  // Check for gears
  let mut gear_symbols: HashMap<XY, Vec<u32>> = HashMap::new();
  for (xy, c) in &schematic.symbols {
    if c == &'*' {
      gear_symbols.insert(*xy, Vec::new());
    }
  }

  for number in &schematic.numbers {
    for x in [number.start.x as i32 - 1, (number.start.x + number.chars) as i32] {
      let xy = XY::create(x as usize, number.start.y);
      if x >= 0 && schematic.symbols.contains_key(&xy) {
        gear_symbols.entry(xy).and_modify(|e| e.push(number.value));
      }
    }
    for y in [number.start.y as i32 - 1, number.start.y as i32 + 1] {
      if y >= 0 {
        for x in (number.start.x as i32 - 1)..=(number.start.x + number.chars) as i32 {
          let xy = XY::create(x as usize, y as usize);
          if x >= 0 && schematic.symbols.contains_key(&xy) {
            gear_symbols.entry(xy).and_modify(|e| e.push(number.value));
          }
        }
      }
    }
  }

  gear_symbols.iter().filter_map(|(_, v)| {
    if v.len() == 2 {
      Some(v[0] * v[1])
    } else {
      None
    }
  })
    .sum()
}


pub fn generator(input: &str) -> Schematic {
  let mut symbols = HashMap::new();
  let mut numbers = Vec::new();
  let mut number: Number = Number::empty();

  for (row, line) in input.lines().enumerate() {
    for (col, c) in line.chars().enumerate() {
      match c {
        c if c.is_ascii_digit() => {
          if number.is_empty() {
            number.set_xy(col, row);
          }
          number.add_digit(c.to_digit(10).unwrap());
        }
        '.' => {
          if !number.is_empty() {
            numbers.push(number);
            number.clear();
          }
        }
        s => {
          if !number.is_empty() {
            numbers.push(number);
            number.clear();
          }
          symbols.insert(XY::create(col, row), s);
        }
      }
    }
  }

  Schematic {
    numbers,
    symbols,
  }
}

#[cfg(test)]
mod tests {
  use crate::day3::{generator, Number, part1, part2, XY};

  fn input() -> String {
    [
      "467..114..",
      "...*......",
      "..35..633.",
      "......#...",
      "617*......",
      ".....+.58.",
      "..592.....",
      "......755.",
      "...$.*....",
      ".664.598..",
    ].join("\n").to_string()
  }

  #[test]
  fn test_generator() {
    let schematic = generator(&input());
    assert_eq!(10, schematic.numbers.len());
    assert_eq!(6, schematic.symbols.len());
    assert_eq!(Number { start: XY::create(0, 0), chars: 3, value: 467 },
               schematic.numbers[0]);
    assert_eq!(Number { start: XY::create(5, 0), chars: 3, value: 114 },
               schematic.numbers[1]);
    assert_eq!(Number { start: XY::create(0, 4), chars: 3, value: 617 },
               schematic.numbers[4]);

    assert_eq!('*', *schematic.symbols.get(&XY::create(3, 1)).unwrap());
    assert_eq!('#', *schematic.symbols.get(&XY::create(6, 3)).unwrap());
  }

  #[test]
  fn test_part1() {
    let schematic = generator(&input());
    assert_eq!(4361, part1(&schematic));
  }

  #[test]
  fn test_part2() {
    let schematic = generator(&input());
    assert_eq!(467835, part2(&schematic));
  }
}
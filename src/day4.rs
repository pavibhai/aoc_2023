use std::cmp::Ordering;
use Ordering::{Less, Equal, Greater};

fn extract_sorted_numbers(input: &str) -> Vec<u32> {
  let mut numbers: Vec<u32> = input.split(' ')
    .filter_map(|tok| {
      if tok.is_empty() { None } else { Some(tok.parse().unwrap()) }
    }).collect();
  numbers.sort();
  numbers
}

pub struct Card {
  numbers: Vec<u32>,
  wins: Vec<u32>,
}

impl Card {
  fn create(line: &str) -> Card {
    let (_, numbers) = line.split_once(':').unwrap();
    let (numbers, wins) = numbers.split_once('|').unwrap();
    let numbers = extract_sorted_numbers(numbers);
    let wins = extract_sorted_numbers(wins);
    Card {
      numbers,
      wins,
    }
  }

  fn winning_numbers(&self) -> u32 {
    let mut winning_numbers = 0;
    let mut i_n = 0;
    let mut i_w = 0;

    while i_n < self.numbers.len() && i_w < self.wins.len() {
      match self.numbers[i_n].cmp(&self.wins[i_w]) {
        Less => i_n += 1,
        Greater => i_w += 1,
        Equal => {
          winning_numbers += 1;
          i_n += 1;
          i_w += 1;
        }
      };
    }

    winning_numbers
  }
}

pub fn generator(input: &str) -> Vec<Card> {
  input.lines()
    .map(Card::create).collect()
}

pub fn part1(cards: &[Card]) -> u32 {
  cards.iter()
    .filter_map(|c| match c.winning_numbers() {
      0 => None,
      v => Some(2u32.pow(v - 1)),
    }).sum()
}

pub fn part2(cards: &[Card]) -> u32 {
  let winning_numbers: Vec<u32> = cards.iter().map(|c| c.winning_numbers()).collect();
  let mut final_counts = vec![1; winning_numbers.len()];

  for (idx, &win_cnt) in winning_numbers.iter().enumerate() {
    for i in (idx + 1)..=(idx + win_cnt as usize).min(winning_numbers.len()) {
      final_counts[i] += final_counts[idx];
    }
  }
  final_counts.iter().sum()
}

#[cfg(test)]
mod tests {
  use crate::day4::{generator, part1, part2};

  fn input() -> String {
    [
      "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
      "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
      "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
      "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
      "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
      "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ].join("\n").to_string()
  }

  #[test]
  fn test_generator() {
    let cards = generator(&input());
    assert_eq!(6, cards.len());
    assert_eq!(&17, cards[0].numbers.first().unwrap());
    assert_eq!(&86, cards[0].numbers.last().unwrap());
    assert_eq!(&6, cards[0].wins.first().unwrap());
    assert_eq!(&86, cards[0].wins.last().unwrap());
  }

  #[test]
  fn test_part1() {
    let cards = generator(&input());
    assert_eq!(13, part1(&cards));
  }

  #[test]
  fn test_part2() {
    let cards = generator(&input());
    assert_eq!(30, part2(&cards));
  }
}
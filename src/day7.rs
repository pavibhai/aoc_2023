use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use crate::day7::Card::{A, J, K, N, Q, T};
use crate::day7::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug, Hash)]
pub enum Card {
  N(u8),
  T,
  J,
  Q,
  K,
  A,
}

impl Card {
  fn from(c: char) -> Card {
    match c {
      'A' => A,
      'K' => K,
      'Q' => Q,
      'J' => J,
      'T' => T,
      c if c.is_ascii_digit() => N(c.to_digit(10).unwrap() as u8),
      _ => panic!("Unexpected card character {c}"),
    }
  }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeKind,
  FullHouse,
  FourKind,
  FiveKind,
}

impl HandType {
  fn determine(cards: &Vec<Card>) -> HandType {
    let mut counts: HashMap<Card, u8> = HashMap::new();
    for card in cards {
      counts.entry(*card)
        .and_modify(|v| *v += 1)
        .or_insert(1);
    }

    match counts.len() {
      1 => FiveKind,
      5 => HighCard,
      4 => OnePair,
      2 => {
        match counts.values().next().unwrap() {
          1 | 4 => FourKind,
          _ => FullHouse,
        }
      }
      3 => {
        match counts.values().max().unwrap() {
          3 => ThreeKind,
          _ => TwoPair,
        }
      }
      _ => panic!("Unexpected hand {:?}", cards),
    }
  }

  fn pretend(&self, jokers: u8) -> HandType {
    match (self, jokers) {
      (FourKind, 1 | 4) => FiveKind,
      (FullHouse, 2 | 3) => FiveKind,
      (ThreeKind, 1 | 3) => FourKind,
      (TwoPair, 1) => FullHouse,
      (TwoPair, 2) => FourKind,
      (OnePair, 1 | 2) => ThreeKind,
      (HighCard, 1) => OnePair,
      _ => *self,
    }
  }
}

#[derive(Eq, Clone, PartialEq)]
pub struct Hand {
  cards: Vec<Card>,
  hand_type: HandType,
  bid: u32,
}

impl Hand {
  fn create(hand: &str) -> Hand {
    let (hand, bid) = hand.split_once(' ').unwrap();
    let cards = hand.chars()
      .map(Card::from)
      .collect();
    Hand {
      hand_type: HandType::determine(&cards),
      cards,
      bid: bid.parse().unwrap(),
    }
  }

  fn jokers(&self) -> u8 {
    self.cards.iter().filter(|&f| f == &J).count() as u8
  }
}

impl PartialOrd<Self> for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.hand_type.cmp(&other.hand_type) {
      Equal => {
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
          match s.cmp(o) {
            Equal => continue,
            o => {
              return o;
            }
          }
        }
        Equal
      }
      o => {
        o
      }
    }
  }
}

pub fn generator(input: &str) -> Vec<Hand> {
  input.lines()
    .map(Hand::create).collect()
}

pub fn part1(hands: &[Hand]) -> u32 {
  let mut sorted_hands = hands.to_vec();
  sorted_hands.sort();

  sorted_hands.iter().enumerate().map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
    .sum()
}

pub fn part2(hands: &[Hand]) -> u32 {
  let mut hands: Vec<Hand> = hands.iter().map(|h| {
    let hand_type = h.hand_type.pretend(h.jokers());
    let cards = h.cards.iter().map(|c| match c {
      J => N(1),
      o => *o,
    }).collect();
    Hand {
      cards,
      hand_type,
      bid: h.bid,
    }
  }).collect();

  hands.sort();

  hands.iter().enumerate().map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
    .sum()
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering::{Greater, Less};
  use crate::day7::Card::{A, K, N, Q};
  use crate::day7::{generator, Hand, part1, part2};
  use crate::day7::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};

  #[test]
  fn test_cards() {
    assert_eq!(A.cmp(&K), Greater);
    assert_eq!(K.cmp(&Q), Greater);
    assert_eq!(Q.cmp(&N(5)), Greater);
  }

  fn input() -> String {
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string()
  }

  #[test]
  fn test_generator() {
    let hands = generator(&input());
    assert_eq!(5, hands.len());
  }

  #[test]
  fn test_hand_types() {
    let hand = Hand::create("32T3K 765");
    assert_eq!(OnePair, hand.hand_type);

    let hand = Hand::create("KK677 28");
    assert_eq!(TwoPair, hand.hand_type);

    let hand = Hand::create("QQQJA 483");
    assert_eq!(ThreeKind, hand.hand_type);

    let hand = Hand::create("QQQAA 483");
    assert_eq!(FullHouse, hand.hand_type);

    let hand = Hand::create("QQQAQ 483");
    assert_eq!(FourKind, hand.hand_type);

    let hand = Hand::create("QQQQQ 483");
    assert_eq!(FiveKind, hand.hand_type);

    let hand = Hand::create("T13AQ 483");
    assert_eq!(HighCard, hand.hand_type);
  }

  #[test]
  fn test_compare() {
    let h1 = Hand::create("T55J5 684");
    let h2 = Hand::create("QQQJA 483");
    assert_eq!(h1.cmp(&h2), Less);

    let hands = generator(&input());
    let mut items = hands.clone();
    items.sort();

    assert_eq!(N(2).cmp(&N(3)), Less);

    let h1 = Hand::create("KK677 28");
    let h2 = Hand::create("KTJJT 220");
    assert_eq!(h1.cmp(&h2), Greater);

    //assert_eq!(items[0].hand_type, HighCard);
  }

  #[test]
  fn test_part1() {
    let hands = generator(&input());
    assert_eq!(6440, part1(&hands));
  }

  #[test]
  fn test_part2() {
    let hands = generator(&input());
    assert_eq!(5905, part2(&hands));
  }
}
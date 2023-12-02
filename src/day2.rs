const RED: &str = "red";
const BLUE: &str = "blue";
const GREEN: &str = "green";

#[derive(Debug)]
pub struct Reveal {
  red: u32,
  blue: u32,
  green: u32,
}

impl Reveal {
  fn create(line: &str) -> Reveal {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    for v in line.split(", ") {
      let mut splits = v.split(' ');
      match (splits.next().unwrap().parse().unwrap(), splits.next().unwrap()) {
        (n, RED) => red = n,
        (n, BLUE) => blue = n,
        (n, GREEN) => green = n,
        (_, c) => panic!("Unexpected color {}", c),
      }
    }

    Reveal {
      red,
      blue,
      green,
    }
  }

  fn exceeds(&self, other: &Reveal) -> bool {
    self.red > other.red || self.blue > other.blue || self.green > other.green
  }

  fn power(&self) -> u32 {
    self.red * self.green * self.blue
  }
}

#[derive(Debug)]
pub struct Game {
  id: u32,
  reveals: Vec<Reveal>,
}

impl Game {
  fn create(line: &str) -> Game {
    let mut splits = line.split(": ");
    let id: u32 = {
      let mut game_splits = splits.next().unwrap().split(' ');
      game_splits.next();
      game_splits.next().unwrap().parse().unwrap()
    };
    let reveals = splits.next().unwrap().split("; ")
      .map(|line| Reveal::create(line))
      .collect();
    Game {
      id,
      reveals,
    }
  }

  fn impossible(&self, max_reveal: &Reveal) -> bool {
    self.reveals.iter().any(|r| r.exceeds(max_reveal))
  }

  fn min_cubes(&self) -> Reveal {
    Reveal {
      red: self.reveals.iter().max_by_key(|r| r.red).unwrap().red,
      blue: self.reveals.iter().max_by_key(|r| r.blue).unwrap().blue,
      green: self.reveals.iter().max_by_key(|r| r.green).unwrap().green,
    }
  }
}

pub fn generator(input: &str) -> Vec<Game> {
  input.lines()
    .map(|line| Game::create(line))
    .collect()
}

pub fn part1(games: &Vec<Game>) -> u32 {
  let max_reveal = Reveal {
    red: 12,
    green: 13,
    blue: 14,
  };

  games.iter().filter_map(|g| if g.impossible(&max_reveal) {
    None
  } else {
    Some(g.id)
  }).sum()
}

pub fn part2(games: &Vec<Game>) -> u32 {
  games.iter().map(|g| g.min_cubes().power()).sum()
}

#[cfg(test)]
mod tests {
  use crate::day2::{generator, part1, part2};

  fn input() -> String {
    [
      "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
      "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
      "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
      "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
      "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ].join("\n")
      .to_string()
  }

  #[test]
  fn test_generator() {
    let games = generator(&input());
    assert_eq!(5, games.len());
    assert_eq!(3, games[0].reveals[0].blue);
    assert_eq!(4, games[0].reveals[0].red);
    assert_eq!(0, games[0].reveals[0].green);
    assert_eq!(2, games[4].reveals[1].blue);
    assert_eq!(1, games[4].reveals[1].red);
    assert_eq!(2, games[4].reveals[1].green);
  }

  #[test]
  fn test_part1() {
    let games = generator(&input());
    assert_eq!(8, part1(&games));
  }

  #[test]
  fn test_part2() {
    let games = generator(&input());
    assert_eq!(2286, part2(&games));
  }
}
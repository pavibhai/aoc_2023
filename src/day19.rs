use std::collections::HashMap;
use crate::day19::Condition::{Else, GreaterThan, LessThan};
use crate::day19::Outcome::{Accepted, Goto, Rejected};

pub fn generator(input: &str) -> Puzzle {
  Puzzle::from(input)
}

pub fn part1(puzzle: &Puzzle) -> u64 {
  puzzle.ratings.iter().filter_map(|r| {
    if puzzle.workflows.accepts(r) {
      Some(r.total())
    } else {
      None
    }
  }).sum()
}

pub fn part2(puzzle: &Puzzle) -> u64 {
  let rr = Rating {
    x: Range { low: 1, high: 4000 },
    m: Range { low: 1, high: 4000 },
    a: Range { low: 1, high: 4000 },
    s: Range { low: 1, high: 4000 },
    exhausted: false,
  };

  puzzle.workflows.accepts_range(&rr)
    .iter().map(|r| r.combinations())
    .sum()
}

pub struct Puzzle {
  workflows: Workflows,
  ratings: Vec<Rating>,
}

impl Puzzle {
  fn from(input: &str) -> Puzzle {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();
    let ratings = ratings.lines().map(Rating::from).collect();

    let id_map = workflows.lines().enumerate().map(|(id, line)| {
      (&line[..line.find(|c| c == '{').unwrap()], id)
    }).collect();
    let flows = workflows.lines().map(|w| {
      Workflow::from(w, &id_map)
    }).collect();

    let workflows = Workflows { start: *id_map.get("in").unwrap(), flows };
    Puzzle { ratings, workflows }
  }
}

struct Workflows {
  start: usize,
  flows: Vec<Workflow>,
}

impl Workflows {
  fn accepts(&self, rating: &Rating) -> bool {
    let mut outcome = &Goto(self.start);
    let mut rr = *rating;
    let mut stack = Vec::new();
    while outcome != &Rejected && outcome != &Accepted {
      self.flows[*outcome.goto_id()].take_evaluate(&mut rr, &mut stack);
      match stack.pop() {
        Some((r, o)) => {
          rr = r;
          outcome = o;
        }
        None => {
          panic!("Unexpected state")
        }
      }
    }
    outcome == &Accepted
  }

  fn accepts_range(&self, rating: &Rating) -> Vec<Rating> {
    let outcome = &Goto(self.start);
    let mut stack = Vec::new();
    let mut accepted: Vec<Rating> = Vec::new();
    stack.push((*rating, outcome));

    while !stack.is_empty() {
      match stack.pop().unwrap() {
        (rr, &Accepted) => accepted.push(rr),
        (_, &Rejected) => {}
        (mut rr, &Goto(id)) => self.flows[id].take_evaluate(&mut rr, &mut stack),
      }
    }

    accepted
  }
}

struct Workflow {
  branches: Vec<Branch>,
}

impl Workflow {
  fn from(input: &str, id_map: &HashMap<&str, usize>) -> Workflow {
    let start = input.chars().position(|c| c == '{').unwrap();
    let len = input.len();
    let branches = input[start + 1..len - 1].split(',')
      .map(|b| Branch::from(b, id_map))
      .collect();
    Workflow { branches }
  }

  fn take_evaluate<'a>(&'a self, rr: &mut Rating, stack: &mut Vec<(Rating, &'a Outcome)>) {
    for branch in &self.branches {
      match branch.take_evaluate(rr) {
        None => {}
        Some(o) => stack.push(o),
      }
      if rr.exhausted { break; }
    }
  }
}


struct Branch {
  cond: Condition,
  outcome: Outcome,
}

impl Branch {
  fn from(input: &str, id_map: &HashMap<&str, usize>) -> Branch {
    let cond;
    let outcome;
    match input.split_once(':') {
      Some((c, o)) => {
        cond = Condition::from(c);
        outcome = Outcome::from(o, id_map);
      }
      None => {
        cond = Else;
        outcome = Outcome::from(input, id_map);
      }
    }

    Branch { cond, outcome }
  }

  fn take_evaluate(&self, rr: &mut Rating) -> Option<(Rating, &Outcome)> {
    self.cond.take_accept(rr).map(|v| (v, &self.outcome))
  }
}

#[derive(Eq, PartialEq, Debug)]
enum Outcome {
  Accepted,
  Rejected,
  Goto(usize),
}

impl Outcome {
  fn from(input: &str, id_map: &HashMap<&str, usize>) -> Outcome {
    match input {
      "A" => Accepted,
      "R" => Rejected,
      g => Goto(*id_map.get(g).unwrap()),
    }
  }

  fn goto_id(&self) -> &usize {
    match self {
      Goto(id) => id,
      _ => panic!("Cannot retrieve id for non goto outcome"),
    }
  }
}

enum Condition {
  LessThan(char, Box<dyn Fn(&mut Rating) -> &mut Range>, u32),
  GreaterThan(char, Box<dyn Fn(&mut Rating) -> &mut Range>, u32),
  Else,
}

impl Condition {
  fn from(input: &str) -> Condition {
    match (input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap()) {
      (c, '<') => LessThan(c, Rating::get_fn(&c),
                           input[2..].parse().unwrap()),
      (c, '>') => GreaterThan(c, Rating::get_fn(&c),
                              input[2..].parse().unwrap()),
      _ => Else,
    }
  }

  fn take_accept(&self, rr: &mut Rating) -> Option<Rating> {
    let mut r = *rr;
    match self {
      LessThan(_,  f, v) => {
        let category = f(rr);
        let accept_category = f(&mut r);
        if &category.low >= v {
          None
        } else if &category.high < v {
          rr.exhaust();
          Some(r)
        } else {
          accept_category.high = v - 1;
          category.low = *v;
          Some(r)
        }
      }
      GreaterThan(_, f, v) => {
        let category = f(rr);
        let accept_category = f(&mut r);
        if &category.high <= v {
          None
        } else if &category.low > v {
          rr.exhaust();
          Some(r)
        } else {
          accept_category.low = v + 1;
          category.high = *v;
          Some(r)
        }
      }
      Else => {
        let r = Some(*rr);
        rr.exhaust();
        r
      }
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Range {
  low: u32,
  high: u32,
}

impl Range {
  fn create(low: u32, high: u32) -> Range {
    Range { low, high }
  }

  fn len(&self) -> u64 {
    (self.high - self.low + 1) as u64
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Rating {
  x: Range,
  m: Range,
  a: Range,
  s: Range,
  exhausted: bool,
}

impl Rating {
  fn from(input: &str) -> Rating {
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;
    let p: &[_] = &['{', '}'];
    input.trim_matches(p)
      .split(',')
      .for_each(|v| match v.split_once('=').unwrap() {
        ("x", v) => x = v.parse().unwrap(),
        ("m", v) => m = v.parse().unwrap(),
        ("a", v) => a = v.parse().unwrap(),
        ("s", v) => s = v.parse().unwrap(),
        (r, _) => panic!("Unexpected rating {}", r),
      });
    Rating::create(x, m, a, s)
  }

  fn create(x: u32, m: u32, a: u32, s: u32) -> Rating {
    Rating {
      x: Range::create(x, x),
      m: Range::create(m, m),
      a: Range::create(a, a),
      s: Range::create(s, s),
      exhausted: false,
    }
  }
  fn exhaust(&mut self) {
    self.exhausted = true;
  }

  fn get_mut_x(&mut self) -> &mut Range {
    &mut self.x
  }

  fn get_mut_m(&mut self) -> &mut Range {
    &mut self.m
  }

  fn get_mut_a(&mut self) -> &mut Range {
    &mut self.a
  }

  fn get_mut_s(&mut self) -> &mut Range {
    &mut self.s
  }

  fn get_fn(c: &char) -> Box<dyn Fn(&mut Rating) -> &mut Range> {
    match c {
      'x' => Box::from(Rating::get_mut_x),
      'm' => Box::from(Rating::get_mut_m),
      'a' => Box::from(Rating::get_mut_a),
      's' => Box::from(Rating::get_mut_s),
      _ => panic!(""),
    }
  }

  fn total(&self) -> u64 {
    (self.x.low + self.m.low + self.a.low + self.s.low) as u64
  }

  fn combinations(&self) -> u64 {
    self.x.len() * self.m.len() * self.a.len() * self.s.len()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::day19::{Condition, generator, Outcome, part1, part2, Range, Rating, Workflow};
  use crate::day19::Outcome::Accepted;

  fn input() -> String {
    "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".to_string()
  }

  #[test]
  fn test_rating() {
    let r = Rating::from("{x=787,m=2655,a=1222,s=2876}");
    assert_eq!(787, r.x.low);
    assert_eq!(2655, r.m.low);
    assert_eq!(1222, r.a.low);
    assert_eq!(2876, r.s.low);
  }

  #[test]
  fn test_generator() {
    let p = generator(&input());
    assert_eq!(p.ratings.len(), 5);
    assert_eq!(p.workflows.flows.len(), 11);
    assert_eq!(p.workflows.start, 7);

    assert!(p.workflows.accepts(&p.ratings[0]));
    assert!(!p.workflows.accepts(&p.ratings[1]));
    assert!(p.workflows.accepts(&p.ratings[2]));
  }

  #[test]
  fn test_part1() {
    let p = generator(&input());
    assert_eq!(19114, part1(&p));
  }

  #[test]
  fn test_part2() {
    let p = generator(&input());
    assert_eq!(167409079868000, part2(&p));
  }

  fn create_range() -> Rating {
    Rating {
      x: Range { low: 1, high: 10 },
      m: Range { low: 1, high: 10 },
      a: Range { low: 1, high: 10 },
      s: Range { low: 1, high: 10 },
      exhausted: false,
    }
  }

  #[test]
  fn test_rating_ranges() {
    let mut rr = create_range();
    let c = Condition::from("x>5");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted.unwrap().x.low, 6);
    assert_eq!(accepted.unwrap().x.high, 10);
    assert_eq!(rr.x.low, 1);
    assert_eq!(rr.x.high, 5);

    rr = create_range();
    let c = Condition::from("x<5");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted.unwrap().x.low, 1);
    assert_eq!(accepted.unwrap().x.high, 4);
    assert_eq!(rr.x.low, 5);
    assert_eq!(rr.x.high, 10);

    rr = create_range();
    let c = Condition::from("x<11");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted.unwrap().x.low, 1);
    assert_eq!(accepted.unwrap().x.high, 10);
    assert!(rr.exhausted);

    rr = create_range();
    let c = Condition::from("x>0");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted.unwrap().x.low, 1);
    assert_eq!(accepted.unwrap().x.high, 10);
    assert!(rr.exhausted);

    rr = create_range();
    let c = Condition::from("x>10");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted, None);
    assert_eq!(rr.x.low, 1);
    assert_eq!(rr.x.high, 10);

    rr = create_range();
    let c = Condition::from("x<1");
    let accepted = c.take_accept(&mut rr);
    assert_eq!(accepted, None);
    assert_eq!(rr.x.low, 1);
    assert_eq!(rr.x.high, 10);
  }

  #[test]
  fn test_range_workflow() {
    let id_map = HashMap::from([
      ("px", 1), ("qkq", 2), ("rfg", 3), ("pv", 4)
    ]);

    let w = Workflow::from("px{a<2006:qkq,m>2090:A,rfg}", &id_map);
    let mut stack: Vec<(Rating, &Outcome)> = Vec::new();
    let mut rr = create_range();
    w.take_evaluate(&mut rr, &mut stack);
    assert!(rr.exhausted);
    assert_eq!(1, stack.len());
    assert_eq!(stack[0].1.goto_id(), &2);
    assert_eq!(stack[0].0, create_range());

    let w = Workflow::from("px{a<5:qkq,m>8:A,rfg}", &id_map);
    stack.clear();
    let mut rr = create_range();
    w.take_evaluate(&mut rr, &mut stack);
    assert!(rr.exhausted);
    assert_eq!(3, stack.len());
    assert_eq!(stack[0].1.goto_id(), &2);
    assert_eq!(stack[0].0,
               Rating {
                 x: Range { low: 1, high: 10 },
                 m: Range { low: 1, high: 10 },
                 a: Range { low: 1, high: 4 },
                 s: Range { low: 1, high: 10 },
                 exhausted: false,
               });
    assert_eq!(stack[1].1, &Accepted);
    assert_eq!(stack[1].0,
               Rating {
                 x: Range { low: 1, high: 10 },
                 m: Range { low: 9, high: 10 },
                 a: Range { low: 5, high: 10 },
                 s: Range { low: 1, high: 10 },
                 exhausted: false,
               });
    assert_eq!(stack[2].1.goto_id(), &3);
    assert_eq!(stack[2].0,
               Rating {
                 x: Range { low: 1, high: 10 },
                 m: Range { low: 1, high: 8 },
                 a: Range { low: 5, high: 10 },
                 s: Range { low: 1, high: 10 },
                 exhausted: false,
               });
  }
}


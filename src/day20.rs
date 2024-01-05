use std::collections::{HashMap, HashSet, VecDeque};
use num::Integer;
use crate::day20::ModType::{Broadcaster, Conjunction, UnTyped, FlipFlop};
use crate::day20::Pulse::{High, Low};

pub fn generator(input: &str) -> Relays {
  Relays::from(input)
}

pub fn part1(relays: &Relays) -> u64 {
  let mut relays = relays.clone();
  let (low, high) = relays.push_button(1000);
  low * high
}

pub fn part2(relays: &Relays) -> u64 {
  let mut relays = relays.clone();
  relays.check_rx_pulses()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Pulse {
  Low,
  High,
}

#[derive(Clone)]
struct Message {
  pulse: Pulse,
  src: usize,
  dest: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ModType {
  Broadcaster,
  FlipFlop,
  Conjunction(u64),
  UnTyped,
}

#[derive(Clone)]
struct Module<'a> {
  id: usize,
  name: &'a str,
  state: u64,
  mod_type: ModType,
  dests: Vec<usize>,
}

impl Module<'_> {
  fn receive(&mut self, message: Message, queue: &mut VecDeque<Message>) {
    let pulse: Option<Pulse> = match self.mod_type {
      Broadcaster => Some(message.pulse),
      FlipFlop => self.receive_flip_flop(message),
      Conjunction(expected) => Some(self.receive_conjunction(expected, message)),
      UnTyped => None,
    };

    if let Some(pulse) = pulse {
      for dest in &self.dests {
        queue.push_back(Message { src: self.id, pulse, dest: *dest })
      }
    }
  }

  fn receive_flip_flop(&mut self, message: Message) -> Option<Pulse> {
    if message.pulse == High {
      return None;
    }

    if self.state == 0 {
      self.state = 1;
      Some(High)
    } else {
      self.state = 0;
      Some(Low)
    }
  }

  fn receive_conjunction(&mut self, expected: u64, message: Message) -> Pulse {
    if message.pulse == High {
      self.state |= 1 << message.src;
    } else {
      self.state &= !(1 << message.src);
    }
    if self.state == expected {
      Low
    } else {
      High
    }
  }

  fn create_conjunction(id: usize, name: &str, srcs: Vec<usize>, dests: Vec<usize>) -> Module {
    let mut expected = 0_u64;
    for src in srcs {
      assert!(src < 64);
      expected |= 1 << src;
    }
    Module { id, name, state: 0, mod_type: Conjunction(expected), dests }
  }

  fn create_flip_flop(id: usize, name: &str, dests: Vec<usize>) -> Module {
    Module { id, name, dests, state: 0, mod_type: FlipFlop }
  }
}

#[derive(Clone)]
pub struct Relays<'a> {
  broadcaster: usize,
  rx: Option<usize>,
  modules: Vec<Module<'a>>,
}

impl Relays<'_> {
  fn make_id_map(input: &str) -> HashMap<&str, usize> {
    let mut id_map: HashMap<&str, usize> = HashMap::new();
    let mut id_value = 0;

    for line in input.lines() {
      let (src, dests) = line.split_once(" -> ").unwrap();
      let name = match src.chars().next().unwrap() {
        '%' | '&' => &src[1..],
        _ => src
      };
      if !id_map.contains_key(name) {
        id_map.insert(name, id_value);
        id_value += 1;
      }

      // Check destination names
      for dest in dests.split(", ") {
        if !id_map.contains_key(dest) {
          id_map.insert(dest, id_value);
          id_value += 1;
        }
      }
    }
    id_map
  }

  fn find_srcs(input: &str) -> (HashMap<&str, Vec<&str>>, HashSet<&str>) {
    let mut srcs = HashSet::new();
    let mut conjunction_srcs = HashMap::new();

    for line in input.lines() {
      let (src, _) = line.split_once(" -> ").unwrap();
      let name = match src.chars().next().unwrap() {
        '%' => &src[1..],
        '&' => {
          let c = &src[1..];
          conjunction_srcs.insert(c, Vec::new());
          c
        }
        _ => src
      };
      srcs.insert(name);
    }
    (conjunction_srcs, srcs)
  }

  fn from(input: &str) -> Relays {
    let id_map = Relays::make_id_map(input);
    let (mut conjunction_srcs, srcs) = Relays::find_srcs(input);

    // Handle conjunction sources
    for line in input.lines() {
      let (src, dests) = line.split_once(" -> ").unwrap();
      let src = match src.chars().next().unwrap() {
        '%' | '&' => &src[1..],
        _ => src
      };
      for dest in dests.split(", ") {
        if conjunction_srcs.contains_key(dest) {
          conjunction_srcs.entry(dest)
            .and_modify(|v| v.push(src));
        }
      }
    }

    let get_id = |v: &str| -> usize {
      *id_map.get(v).unwrap()
    };
    let get_dests = |dests: &str| -> Vec<usize> {
      dests.split(", ")
        .map(|v| *id_map.get(v).unwrap())
        .collect()
    };

    // Create modules
    let mut modules: Vec<Module> = Vec::new();
    for line in input.lines() {
      let (src, dests) = line.split_once(" -> ").unwrap();
      match src.chars().next().unwrap() {
        '%' => {
          let id = get_id(&src[1..]);
          let dests = get_dests(dests);
          modules.push(Module::create_flip_flop(id, &src[1..], dests));
        }
        '&' => {
          let id = get_id(&src[1..]);
          let dests = get_dests(dests);
          let srcs = conjunction_srcs.get(&src[1..]).unwrap()
            .iter().map(|&v| *id_map.get(v).unwrap())
            .collect();
          modules.push(Module::create_conjunction(id, &src[1..], srcs, dests));
        }
        _ if src == "broadcaster" => {
          let id = get_id(src);
          let dests = get_dests(dests);
          modules.push(Module { id, name: src, dests, state: 0, mod_type: Broadcaster });
        }
        _ => panic!("Unexpected value {src}"),
      }
    }
    // Handle untyped modules
    for (name, id) in &id_map {
      if !srcs.contains(name) {
        modules.push(Module { id: *id, name, dests: vec![], state: 0, mod_type: UnTyped });
      }
    }
    modules.sort_by_key(|m| m.id);
    Relays::create(*id_map.get("broadcaster").unwrap(),
                   id_map.get("rx").copied(),
                   modules)
  }
  fn create(broadcaster: usize, rx: Option<usize>, modules: Vec<Module>) -> Relays {
    Relays { broadcaster, rx, modules }
  }
  fn push_button(&mut self, times: u32) -> (u64, u64) {
    let mut messages = VecDeque::new();
    let mut low_pulses = 0_u64;
    let mut high_pulses = 0_u64;
    for _ in 0..times {
      let message = Message { src: self.broadcaster, dest: self.broadcaster, pulse: Low };
      messages.push_back(message);

      while !messages.is_empty() {
        let message = messages.pop_front().unwrap();
        if message.pulse == Low {
          low_pulses += 1;
        } else {
          high_pulses += 1;
        }
        self.modules[message.dest].receive(message, &mut messages);
      }
    }
    (low_pulses, high_pulses)
  }

  fn find_sources_for(&self, tgt: usize) -> Vec<usize> {
    self.modules.iter().filter_map(|m| {
      if m.dests.contains(&tgt) {
        Some(m.id)
      } else {
        None
      }
    }).collect()
  }

  fn identify_flows(&self) -> (Vec<usize>, Vec<usize>) {
    let starts: Vec<usize> = self.modules[self.broadcaster].dests.to_vec();
    let mut ends = self.find_sources_for(self.rx.unwrap());
    assert_eq!(ends.len(), 1);
    match self.modules[ends[0]].mod_type {
      Conjunction(_) => {},
      _ => {panic!("Expected a conjunction type")}
    };
    ends = self.find_sources_for(ends[0]);
    assert_eq!(ends.len(), starts.len());

    (starts, ends)
  }

  fn check_rx_pulses(&mut self) -> u64 {
    // Identify independent flows
    let (starts, ends) = self.identify_flows();
    let sub_flows = self.check_mutually_exclusive(&starts, &ends);

    // Each of these flows should output a high for us to get the desired output
    sub_flows.iter()
      .map(|(s, e)| {
        let (from, repeat, highs) = self.find_repeat(s, e);
        assert_eq!(from, 0);
        assert_eq!(highs.len(), 1);
        assert_eq!(highs[0] + 1, repeat);
        highs[0] as u64 + 1
      }).fold(1, |a, v| a.lcm(&v))
  }

  fn check_mutually_exclusive(&self, starts: &[usize], ends: &[usize]) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    let mut scopes = vec![0_u64; starts.len()];
    let mut stack = Vec::new();

    for (idx, start) in starts.iter().enumerate() {
      stack.push(start);
      while !stack.is_empty() {
        let m = stack.pop().unwrap();
        let v = 1_u64 << *m;
        if scopes[idx] & v == 0 {
          scopes[idx] |= v;
          if ends.contains(m) {
            output.push((*start, *m));
            continue;
          };
          for dest in &self.modules[*m].dests {
            stack.push(dest);
          }
        }
      }
    }

    for i in 0..scopes.len() - 1 {
      for j in i + 1..scopes.len() {
        if scopes[i] & scopes[j] != 0 {
          panic!("Overlap detected between {} and {}",
                 self.modules[starts[i]].name,
                 self.modules[starts[j]].name)
        }
      }
    }

    output
  }

  fn find_repeat(&self, start: &usize, end: &usize) -> (usize, usize, Vec<usize>) {
    let mut repeats: HashMap<Vec<u64>, usize> = HashMap::new();
    let mut times = 0;
    let mut modules = self.modules.clone();
    let mut messages: VecDeque<Message> = VecDeque::new();
    let mut high_times = Vec::new();

    loop {
      let mut high_pulses = 0_u32;
      let message = Message { src: *start, dest: *start, pulse: Low };
      messages.push_back(message);

      while !messages.is_empty() {
        let message = messages.pop_front().unwrap();
        if message.src == *end && message.pulse == High {
          high_pulses += 1;
        }
        modules[message.dest].receive(message, &mut messages);
      }

      if high_pulses > 0 {
        high_times.push(times);
      }

      let state: Vec<u64> = modules.iter().map(|m| m.state).collect();
      match repeats.get(&state) {
        None => {
          repeats.insert(state, times);
        }
        Some(v) => {
          return (*v, times, high_times);
        }
      }
      times += 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::VecDeque;
  use crate::day20::{Conjunction, generator, Message, Module, part1, Relays};
  use crate::day20::Pulse::{High, Low};

  #[test]
  fn test_flipflop() {
    let mut m = Module::create_flip_flop(1, "a", vec![0, 5, 6]);
    let mut queue = VecDeque::new();
    m.receive(Message { pulse: High, src: 2, dest: 1 }, &mut queue);
    assert_eq!(0, m.state);
    assert!(queue.is_empty());

    m.receive(Message { pulse: Low, src: 2, dest: 1 }, &mut queue);
    assert_eq!(1, m.state);
    assert_eq!(3, queue.len());
  }

  #[test]
  fn test_conjunction() {
    let mut m = Module::create_conjunction(1, "c", vec![2, 3], vec![5, 9]);
    assert_eq!(m.mod_type, Conjunction(12));
    assert_eq!(m.state, 0);

    let mut queue = VecDeque::new();
    m.receive(Message { pulse: High, src: 2, dest: 1 }, &mut queue);
    assert_eq!(2, queue.len());
    assert!(queue.iter().all(|m| m.pulse == High));

    queue.clear();
    m.receive(Message { pulse: High, src: 3, dest: 1 }, &mut queue);
    assert_eq!(2, queue.len());
    assert!(queue.iter().all(|m| m.pulse == Low));

    queue.clear();
    m.receive(Message { pulse: High, src: 3, dest: 1 }, &mut queue);
    assert_eq!(2, queue.len());
    assert!(queue.iter().all(|m| m.pulse == Low));

    queue.clear();
    m.receive(Message { pulse: Low, src: 3, dest: 1 }, &mut queue);
    assert_eq!(2, queue.len());
    assert!(queue.iter().all(|m| m.pulse == High));
    assert_eq!(4, m.state);
  }

  #[test]
  fn test_relays() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let mut relays = Relays::from(input);
    let (low, high) = relays.push_button(1);
    assert_eq!(8, low);
    assert_eq!(4, high);
  }

  #[test]
  fn test_generator() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let relays = generator(input);
    assert_eq!(0, relays.broadcaster);
    assert_eq!(5, relays.modules.len());
    assert_eq!(0, relays.modules[0].id);
    assert_eq!(4, relays.modules[4].id);
  }

  #[test]
  fn test_part1() {
    let relays = generator("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a");
    assert_eq!(32000000, part1(&relays));

    let relays = generator("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output");
    assert_eq!(11687500, part1(&relays));
  }
}
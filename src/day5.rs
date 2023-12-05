use std::str::Lines;

pub fn generator(input: &str) -> Almanac {
  let mut sections = input.split("\n\n");
  let seeds_section = sections.next().unwrap();
  let seeds = seeds_section
    .split_whitespace()
    .skip(1)
    .map(|e| e.parse().unwrap())
    .collect();

  let mut seed_to_soil = Map::new();
  let mut soil_to_fertilizer = Map::new();
  let mut fertilizer_to_water = Map::new();
  let mut water_to_light = Map::new();
  let mut light_to_temp = Map::new();
  let mut temp_to_humidity = Map::new();
  let mut humidity_to_loc = Map::new();

  for map_section in sections {
    let mut lines = map_section.lines();
    match lines.next().unwrap() {
      "seed-to-soil map:" => seed_to_soil.add_entries(lines),
      "soil-to-fertilizer map:" => soil_to_fertilizer.add_entries(lines),
      "fertilizer-to-water map:" => fertilizer_to_water.add_entries(lines),
      "water-to-light map:" => water_to_light.add_entries(lines),
      "light-to-temperature map:" => light_to_temp.add_entries(lines),
      "temperature-to-humidity map:" => temp_to_humidity.add_entries(lines),
      "humidity-to-location map:" => humidity_to_loc.add_entries(lines),
      header => panic!("Unexpected header {}", header),
    }
  }

  Almanac {
    seeds,
    seed_to_soil,
    soil_to_fertilizer,
    fertilizer_to_water,
    water_to_light,
    light_to_temp,
    temp_to_humidity,
    humidity_to_loc,
  }
}

struct Entry {
  src: u32,
  dst: u32,
  range: u32,
}

impl Entry {
  fn create(input: &str) -> Entry {
    let mut values = input.split_whitespace();
    Entry {
      dst: values.next().unwrap().parse().unwrap(),
      src: values.next().unwrap().parse().unwrap(),
      range: values.next().unwrap().parse().unwrap(),
    }
  }
}

struct Map {
  entries: Vec<Entry>,
}

impl Map {
  fn new() -> Map {
    Map {
      entries: Vec::new()
    }
  }
  fn add_entries(&mut self, lines: Lines) {
    for line in lines {
      self.entries.push(Entry::create(line));
    }

    self.entries.sort_by_key(|e| e.src);
  }

  fn destination(&self, src: &u32) -> u32 {
    match self.entries.binary_search_by_key(src, |e| e.src) {
      Ok(i) => self.entries[i].dst,
      Err(i) if i > 0 => {
        let prev = &self.entries[i - 1];
        match src - prev.src {
          i if i < prev.range => prev.dst + i,
          _ => *src
        }
      }
      Err(_) => *src,
    }
  }

  fn dest_ranges(&self, src_ranges: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut stack: Vec<(u32, u32)> = src_ranges.to_vec();
    let mut dest_ranges: Vec<(u32, u32)> = Vec::new();
    while !stack.is_empty() {
      let (mut src, mut range) = stack.pop().unwrap();
      match self.entries.binary_search_by_key(&src, |e| e.src) {
        Ok(i) => {
          match range as i32 - self.entries[i].range as i32 {
            v if v.is_negative() || v == 0 => {
              dest_ranges.push((self.entries[i].dst, range));
            }
            v => {
              dest_ranges.push((self.entries[i].dst, self.entries[i].range));
              stack.push((self.entries[i].src + self.entries[i].range, v as u32));
            }
          }
        }
        Err(i) => {
          if i > 0 {
            // handle over with previous
            let prev = &self.entries[i - 1];
            match src - prev.src {
              v if v < prev.range => {
                dest_ranges.push((prev.dst + v, range.min(prev.range - v)));
                src = prev.src + prev.range;
                range -= range.min(prev.range - v);
              }
              _ => {}
            }
          }
          if i < self.entries.len() && range > 0 {
            // handle overlap with next
            let next = &self.entries[i];
            match next.src - src {
              v if v < range => {
                dest_ranges.push((src, v));
                src += v;
                range -= v;
                stack.push((src, range));
              }
              _ => {
                dest_ranges.push((src, range));
              }
            }
          }
          if i == self.entries.len() && range > 0 {
            dest_ranges.push((src, range));
          }
        }
      }
    }
    dest_ranges
  }
}

pub struct Almanac {
  seeds: Vec<u32>,
  seed_to_soil: Map,
  soil_to_fertilizer: Map,
  fertilizer_to_water: Map,
  water_to_light: Map,
  light_to_temp: Map,
  temp_to_humidity: Map,
  humidity_to_loc: Map,
}

impl Almanac {
  fn seed_to_loc(&self, seed: &u32) -> u32 {
    self.humidity_to_loc.destination(
      &self.temp_to_humidity.destination(
        &self.light_to_temp.destination(
          &self.water_to_light.destination(
            &self.fertilizer_to_water.destination(
              &self.soil_to_fertilizer.destination(
                &self.seed_to_soil.destination(seed))
            )
          )
        )
      )
    )
  }

  fn seed_ranges_to_loc_ranges(&self, seed_ranges: &[(u32, u32)]) -> Vec<(u32, u32)> {
    self.humidity_to_loc.dest_ranges(
      &self.temp_to_humidity.dest_ranges(
        &self.light_to_temp.dest_ranges(
          &self.water_to_light.dest_ranges(
            &self.fertilizer_to_water.dest_ranges(
              &self.soil_to_fertilizer.dest_ranges(
                &self.seed_to_soil.dest_ranges(seed_ranges))
            )
          )
        )
      )
    )
  }
}

pub fn part1(almanac: &Almanac) -> u32 {
  almanac.seeds.iter().map(|s| {
    almanac.seed_to_loc(s)
  }).min().unwrap()
}

pub fn part2(almanac: &Almanac) -> u32 {
  let mut seed_ranges = Vec::new();
  for i in 0..almanac.seeds.len() / 2 {
    seed_ranges.push((almanac.seeds[2 * i], almanac.seeds[(2 * i) + 1]));
  }
  almanac.seed_ranges_to_loc_ranges(&seed_ranges)
    .iter()
    .min_by_key(|e| e.0)
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
  use crate::day5::{Entry, generator, Map, part1, part2};

  fn input() -> String {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string()
  }

  #[test]
  fn test_generator() {
    let a = generator(&input());
    assert_eq!(a.seeds.len(), 4);
    assert_eq!(a.seed_to_soil.entries.len(), 2);
    assert_eq!(a.soil_to_fertilizer.entries.len(), 3);
    assert_eq!(a.fertilizer_to_water.entries.len(), 4);
    assert_eq!(a.water_to_light.entries.len(), 2);
    assert_eq!(a.light_to_temp.entries.len(), 3);
    assert_eq!(a.temp_to_humidity.entries.len(), 2);
    assert_eq!(a.humidity_to_loc.entries.len(), 2);
  }

  #[test]
  fn test_part1() {
    let a = generator(&input());
    assert_eq!(35, part1(&a));
  }

  #[test]
  fn test_part2() {
    let a = generator(&input());
    assert_eq!(46, part2(&a));
  }

  #[test]
  fn test_dest_ranges() {
    let m = Map {
      entries: vec![
        Entry { src: 5, dst: 50, range: 6 },
        Entry { src: 15, dst: 150, range: 10 },
      ],
    };

    // over and beyond the existing range
    let dest_ranges = m.dest_ranges(&[(1, 30)]);
    assert_eq!(vec![(1, 4), (50, 6), (11, 4), (150, 10), (25, 6)], dest_ranges);

    // only before
    let dest_ranges = m.dest_ranges(&[(4, 1)]);
    assert_eq!(vec![(4, 1)], dest_ranges);

    // only after
    let dest_ranges = m.dest_ranges(&[(25, 1)]);
    assert_eq!(vec![(25, 1)], dest_ranges);

    // in gap
    let dest_ranges = m.dest_ranges(&[(12, 1)]);
    assert_eq!(vec![(12, 1)], dest_ranges);

    // prev, gap, next
    let dest_ranges = m.dest_ranges(&[(8, 8)]);
    assert_eq!(vec![(53, 3), (11, 4), (150, 1)], dest_ranges);

    // gap, next
    let dest_ranges = m.dest_ranges(&[(13, 3)]);
    assert_eq!(vec![(13, 2), (150, 1)], dest_ranges);
  }
}
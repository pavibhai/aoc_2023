use std::collections::{HashMap, VecDeque};


#[derive(Clone)]
pub struct WiringDiagram<'a> {
  id_map: Vec<&'a str>,
  links: Vec<Vec<usize>>,
}

impl WiringDiagram<'_> {
  fn from(input: &str) -> WiringDiagram {
    let mut id_map: HashMap<&str, usize> = HashMap::new();
    for line in input.lines() {
      let (from, to) = line.split_once(": ").unwrap();
      let i = id_map.len();
      id_map.entry(from).or_insert(i);
      for to in to.split(' ') {
        let i = id_map.len();
        id_map.entry(to).or_insert(i);
      }
    }
    let mut links: Vec<Vec<usize>> = vec![Vec::new(); id_map.len()];
    for line in input.lines() {
      let (from, to) = line.split_once(": ").unwrap();
      let l1 = id_map.get(from).unwrap();
      for to in to.split(' ') {
        let l2 = id_map.get(to).unwrap();
        links[*l1].push(*l2);
        links[*l2].push(*l1);
      }
    }

    let mut ids = vec![""; links.len()];
    for (k, v) in id_map {
      ids[v] = k;
    }

    WiringDiagram {
      id_map: ids,
      links,
    }
  }

  fn compute_splits(&self) -> u32 {
    let closest_points = self.find_closest_points();
    let mut wd = self.clone();
    // Determine the edges from the closest 6 components
    let mut edges = Vec::new();
    for i in 0..5 {
      for j in i + 1..6 {
        if self.links[closest_points[i].0].contains(&closest_points[j].0) {
          edges.push((closest_points[i].0, closest_points[j].0));
        }
      }
    }

    for i in 0..edges.len() - 2 {
      wd.disable_link(edges[i].0, edges[i].1);
      for j in i + 1..edges.len() - 1 {
        wd.disable_link(edges[j].0, edges[j].1);
        for k in j + 1..edges.len() {
          wd.disable_link(edges[k].0, edges[k].1);
          if let Some(l) = wd.is_two_groups(edges[i].0, edges[i].1) {
            return l;
          }
          wd.enable_link(edges[k].0, edges[k].1);
        }
        wd.enable_link(edges[j].0, edges[j].1);
      }
      wd.enable_link(edges[i].0, edges[i].1);
    }
    panic!("Could not determine split for the given input!!!")
  }

  fn is_two_groups(&self, from: usize, cannot_reach: usize) -> Option<u32> {
    let mut pending = Vec::new();
    pending.push(from);
    let mut visited = vec![false; self.id_map.len()];

    while !pending.is_empty() {
      let c = pending.pop().unwrap();
      if visited[c] {
        continue;
      }
      visited[c] = true;
      if c == cannot_reach {
        return None;
      }
      for v in &self.links[c] {
        pending.push(*v);
      }
    }
    let group1_size = visited.iter().filter(|&v| *v).count() as u32;
    Some(group1_size * (self.id_map.len() as u32 - group1_size))
  }

  fn find_closest_points(&self) -> Vec<(usize, u32)> {
    let mut all_distances: Vec<(usize, u32)> = (0..self.id_map.len())
      .map(|start| (start, self.find_distances(start).iter().sum()))
      .collect();
    all_distances.sort_by_key(|(_, total_dist)| *total_dist);
    all_distances
  }

  fn find_distances(&self, start: usize) -> Vec<u32> {
    let mut distances = vec![0; self.id_map.len()];
    let mut visited = vec![false; self.id_map.len()];
    let mut queue = VecDeque::new();
    let mut count = 0;

    visited[start] = true;
    count += 1;
    queue.push_back(vec![start]);

    while count < visited.len() {
      let path = queue.pop_front().unwrap();
      for t in &self.links[*path.last().unwrap()] {
        if visited[*t] { continue; }
        visited[*t] = true;
        count += 1;
        let mut p = path.clone();
        p.push(*t);
        distances[*t] = (p.len() - 1) as u32;
        queue.push_back(p);
      }
    }

    distances
  }

  fn enable_link(&mut self, l1: usize, l2: usize) {
    self.links[l1].push(l2);
    self.links[l2].push(l1);
  }

  fn disable_link(&mut self, l1: usize, l2: usize) {
    self.links[l1].retain(|v| v != &l2);
    self.links[l2].retain(|v| v != &l1);
  }
}

pub fn generator(input: &str) -> WiringDiagram {
  WiringDiagram::from(input)
}

pub fn part1(wd: &WiringDiagram) -> u32 {
  wd.compute_splits()
}

pub fn part2(_wd: &WiringDiagram) -> u32 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day25::{generator, part1};

  const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

  #[test]
  fn test_generator() {
    let wd = generator(INPUT);
    assert_eq!(15, wd.links.len());
    assert_eq!("jqt", wd.id_map[0]);
    assert_eq!(4, wd.links[0].len());
    assert_eq!("rhn", wd.id_map[1]);
    assert_eq!(4, wd.links[1].len());
  }

  #[test]
  fn test_part1() {
    let wd = generator(INPUT);
    assert_eq!(part1(&wd), 54);
  }

  #[test]
  fn test_shortest_paths() {
    let wd = generator(INPUT);
    //wd.find_most_used_edges();
    wd.find_closest_points();
  }
}
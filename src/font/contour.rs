use std::collections::{HashSet, VecDeque};

use multimap::MultiMap;

use crate::font::point::{Edge, Point};

pub fn contour(pixels: &[Point]) -> Vec<Vec<Point>> {
  clusters(pixels)
    .into_iter()
    .map(|c| edges(&c))
    .flat_map(|e| connect_edges(&e))
    .collect()
}

fn connect_edges(edges: &[Edge]) -> Vec<Vec<Point>> {
  let mut contours: Vec<Vec<Point>> = Vec::new();
  let map = reduce_edges(edges);

  let keys: Vec<Point> = map.keys().copied().collect();
  let mut visited: HashSet<Edge> = HashSet::new();
  for start in keys {
    let mut contour: Vec<Point> = Vec::new();

    let mut last_edge: Option<Edge> = None;
    let mut current = start;
    loop {
      let candidate = if let Some(last_edge) = last_edge
        && map.is_vec(&current)
      {
        let choices = map.get_vec(&current).expect("Non cyclic");
        if choices.iter().all(|p| visited.contains(&(current, *p))) {
          break;
        }

        let last_sign = sign(last_edge.0, last_edge.1);
        choices
          .iter()
          .filter(|p| !visited.contains(&(current, **p)))
          .find(|p| sign(current, **p) == last_sign)
          .copied()
          .unwrap_or_else(|| {
            panic!(
              "No sign match with edges: {map:?}\nchoices: {choices:?}\nvisited: {visited:?}\ncurrent: {current}\nlast_sign: {last_sign}\nlast_edge: {last_edge:?}"
            )
          })
      } else {
        *map.get(&current).expect("Non cyclic")
      };

      let edge = (current, candidate);
      if visited.contains(&edge) {
        break;
      }

      last_edge = Some(edge);
      visited.insert(edge);
      contour.push(current);
      current = candidate;
    }

    if !contour.is_empty() {
      contours.push(contour);
    }
  }

  contours
}

fn reduce_edges(edges: &[Edge]) -> MultiMap<Point, Point> {
  let mut map: MultiMap<Point, Point> = edges.iter().copied().collect();

  for (a, b) in edges {
    if !map.contains_key(a) || map.is_vec(b) {
      continue;
    }

    let mut droplist: Vec<Point> = vec![];
    let mut mid = *b;
    let mut end;
    loop {
      end = mid;
      if map.is_vec(&mid) {
        break;
      }

      let c = *map.get(&mid).expect("Not cyclic");
      if a.x == c.x || a.y == c.y {
        droplist.push(mid);
        mid = c;
      } else {
        break;
      }
    }

    for p in &droplist {
      map.remove(p);
    }

    if *b != end {
      let b_pointer = map
        .get_vec_mut(a)
        .expect("Not cyclic")
        .iter_mut()
        .find(|x| *x == b)
        .expect("Point is missing");
      *b_pointer = end;
    }
  }

  map
}

fn sign(a: Point, b: Point) -> bool {
  ((b.y - a.y) + (b.x - a.x)) > 0
}

fn edges(cluster: &HashSet<Point>) -> Vec<Edge> {
  cluster
    .iter()
    .flat_map(|p| {
      [
        (!cluster.contains(&p.above())).then(|| p.top_edge()),
        (!cluster.contains(&p.right())).then(|| p.right_edge()),
        (!cluster.contains(&p.below())).then(|| p.bottom_edge()),
        (!cluster.contains(&p.left())).then(|| p.left_edge()),
      ]
    })
    .flatten()
    .collect()
}

fn clusters(pixels: &[Point]) -> Vec<HashSet<Point>> {
  let mut clusters = Vec::new();
  let mut visited: HashSet<Point> = HashSet::new();
  let points: HashSet<Point> = pixels.iter().copied().collect();

  let mut queue: VecDeque<Point> = VecDeque::new();
  for p in &points {
    if visited.contains(p) {
      continue;
    }
    visited.insert(*p);

    let mut cluster: HashSet<Point> = HashSet::new();
    cluster.insert(*p);
    queue.push_back(*p);

    while let Some(p) = queue.pop_front() {
      let candidates: Vec<Point> = p
        .adjacent()
        .into_iter()
        .filter(|p| points.contains(p) && !visited.contains(p))
        .collect();

      for c in candidates {
        visited.insert(c);
        cluster.insert(c);
        queue.push_back(c);
      }
    }

    clusters.push(cluster);
  }

  clusters
}

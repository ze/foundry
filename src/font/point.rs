use std::fmt::{Debug, Display};

pub type Edge = (Point, Point);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
  pub x: i16,
  pub y: i16,
}

impl Point {
  pub fn new(x: i16, y: i16) -> Self {
    Self { x, y }
  }

  pub fn scale(&mut self, scale: i16) {
    self.x *= scale;
    self.y *= scale;
  }

  pub fn above(self) -> Self {
    Point::new(self.x, self.y - 1)
  }

  pub fn below(self) -> Self {
    Point::new(self.x, self.y + 1)
  }

  pub fn left(self) -> Self {
    Point::new(self.x - 1, self.y)
  }

  pub fn right(self) -> Self {
    Point::new(self.x + 1, self.y)
  }

  pub fn adjacent(self) -> Vec<Point> {
    vec![
      Point::new(self.x - 1, self.y),
      Point::new(self.x + 1, self.y),
      Point::new(self.x, self.y - 1),
      Point::new(self.x, self.y + 1),
    ]
  }

  pub fn surrounding(self) -> [Self; 8] {
    [
      Point::new(self.x - 1, self.y - 1),
      Point::new(self.x, self.y - 1),
      Point::new(self.x + 1, self.y - 1),
      Point::new(self.x - 1, self.y),
      Point::new(self.x + 1, self.y),
      Point::new(self.x - 1, self.y + 1),
      Point::new(self.x, self.y + 1),
      Point::new(self.x + 1, self.y + 1),
    ]
  }

  pub fn top_edge(self) -> Edge {
    (self, Point::new(self.x + 1, self.y))
  }

  pub fn right_edge(self) -> Edge {
    (
      Point::new(self.x + 1, self.y),
      Point::new(self.x + 1, self.y + 1),
    )
  }

  pub fn bottom_edge(self) -> Edge {
    (
      Point::new(self.x + 1, self.y + 1),
      Point::new(self.x, self.y + 1),
    )
  }

  pub fn left_edge(self) -> Edge {
    (Point::new(self.x, self.y + 1), self)
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

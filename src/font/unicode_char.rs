use std::{cmp::Ordering, fmt::Display};

const SPACE: char = '\u{0020}';
const NBSP: char = '\u{00A0}';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeChar {
  NotDef,
  Char(char),
}

impl UnicodeChar {
  pub const UNICODE_SPACE: UnicodeChar = UnicodeChar::Char(SPACE);
  pub const UNICODE_NBSP: UnicodeChar = UnicodeChar::Char(NBSP);

  pub fn is_rendering(self) -> bool {
    !self.is_space()
  }

  pub fn is_space(self) -> bool {
    self == UnicodeChar::UNICODE_SPACE || self == UnicodeChar::UNICODE_NBSP
  }
}

impl Display for UnicodeChar {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UnicodeChar::NotDef => write!(f, ".notdef"),
      UnicodeChar::Char(c) => write!(f, "{c}"),
    }
  }
}

impl PartialOrd for UnicodeChar {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for UnicodeChar {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (UnicodeChar::NotDef, UnicodeChar::NotDef) => Ordering::Equal,
      (UnicodeChar::NotDef, UnicodeChar::Char(_)) => Ordering::Less,
      (UnicodeChar::Char(_), UnicodeChar::NotDef) => Ordering::Greater,
      (UnicodeChar::Char(a), UnicodeChar::Char(b)) => a.cmp(b),
    }
  }
}

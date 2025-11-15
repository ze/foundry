use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::font::{glyphs::Glyph, point::Point};

#[derive(Deserialize, Serialize)]
pub struct Kerning {
  #[serde(default = "default_true", skip_serializing_if = "is_true")]
  enabled: bool,
  limit: i16,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pairs: Vec<KerningPair>,
}

impl Kerning {
  pub fn new(enabled: bool, limit: i16, pairs: Vec<KerningPair>) -> Self {
    Self {
      enabled,
      limit,
      pairs,
    }
  }

  pub fn kern(&self, glyphs: &[Glyph]) -> HashMap<(char, char), i16> {
    if !self.enabled {
      return HashMap::with_capacity(0);
    }

    let mut result: HashMap<(char, char), i16> = HashMap::new();

    let chars: HashSet<char> = glyphs
      .iter()
      .filter_map(|g| g.character.try_into().ok())
      .collect();
    let map: HashMap<(char, char), (bool, bool, i16)> = self
      .pairs
      .iter()
      .map(|pair| {
        (
          (pair.left, pair.right),
          (pair.enabled, pair.alts, pair.value),
        )
      })
      .collect();

    for left in glyphs {
      if !left.character.should_kern() {
        continue;
      }

      for right in glyphs {
        if !right.character.should_kern() {
          continue;
        }

        let char_l: char = left.character.try_into().unwrap();
        let char_r: char = right.character.try_into().unwrap();
        if let Some((enabled, alts, value)) = map.get(&(char_l, char_r)) {
          if !enabled {
            continue;
          }

          let kerning = *value;
          if kerning == 0 {
            continue;
          }

          result.insert((char_l, char_r), kerning);
          if *alts {
            let l_alts = Kerning::alts(char_l);
            let r_alts = Kerning::alts(char_r);
            for char_l in l_alts.unwrap_or("").chars().filter(|c| chars.contains(c)) {
              for char_r in r_alts.unwrap_or("").chars().filter(|c| chars.contains(c)) {
                result.insert((char_l, char_r), kerning);
              }
            }
          }
        } else {
          let kerning = self.kern_pair(left, right);
          if kerning != 0 {
            result.insert((char_l, char_r), kerning);
          }
        }
      }
    }

    result
  }

  fn kern_pair(&self, left: &Glyph, right: &Glyph) -> i16 {
    let mut kerning = 0i16;
    let mut offset = left.bbox.x_max + 1;

    while offset > 0 && kerning > self.limit {
      offset -= 1;

      if right
        .pixels
        .iter()
        .map(|p| Point::new(p.x + offset, p.y))
        .any(|p| p.surrounding().iter().any(|s| left.pixels.contains(s)))
      {
        break;
      }

      kerning -= 1;
    }

    kerning
  }

  fn alts(c: char) -> Option<&'static str> {
    match c {
      'A' => Some("ÀÁÂÃÄÅ"),
      'a' => Some("àáâãäå"),
      'C' => Some("Ç"),
      'c' => Some("ç"),
      'E' => Some("ÈÉÊË"),
      'e' => Some("èéêë"),
      'I' => Some("ÌÍÎÏ"),
      'i' => Some("ìíîï"),
      'N' => Some("Ñ"),
      'n' => Some("ñ"),
      'O' => Some("ÒÓÔÕÖ"),
      'o' => Some("òóôõö"),
      'U' => Some("ÙÚÛÜ"),
      'u' => Some("ùúûü"),
      'Y' => Some("Ÿ"),
      'y' => Some("ÿ"),
      _ => None,
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct KerningPair {
  #[serde(default = "default_true", skip_serializing_if = "is_true")]
  pub enabled: bool,
  pub left: char,
  pub right: char,
  pub value: i16,
  #[serde(default = "default_true", skip_serializing_if = "is_true")]
  pub alts: bool,
}

fn default_true() -> bool {
  true
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_true(v: &bool) -> bool {
  *v
}

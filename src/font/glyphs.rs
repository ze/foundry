use constcat::concat;
use read_fonts::tables::glyf::CurvePoint;
use write_fonts::tables::glyf::{Bbox, SimpleGlyph};

use crate::font::{point::Point, unicode_char::UnicodeChar};

pub mod sheet_unicode_blocks {
  const BASIC_LATIN_CODE_POINTS: u32 = 128;
  const BASIC_LATIN_CONTROLS: u32 = 33;
  const BASIC_LATIN_NON_RENDERING: u32 = 1; // space 0x20
  pub const BASIC_LATIN_SUPPORTED: u32 =
    BASIC_LATIN_CODE_POINTS - BASIC_LATIN_CONTROLS - BASIC_LATIN_NON_RENDERING;

  const LATIN_SUPPLEMENT_CODE_POINTS: u32 = 128;
  const LATIN_SUPPLEMENT_CONTROLS: u32 = 32;
  const LATIN_SUPPLEMENT_NON_RENDERING: u32 = 2; // nbsp 0xa0, soft-hyphen 0xad
  pub const LATIN_SUPPLEMENT_SUPPORTED: u32 =
    LATIN_SUPPLEMENT_CODE_POINTS - LATIN_SUPPLEMENT_CONTROLS - LATIN_SUPPLEMENT_NON_RENDERING;

  const LATIN_A_CODE_POINTS: u32 = 128;
  const LATIN_A_CONTROLS: u32 = 0;
  const LATIN_A_UNSUPPORTED: u32 = 1; // long s 0x17f
  pub const LATIN_A_SUPPORTED: u32 = LATIN_A_CODE_POINTS - LATIN_A_CONTROLS - LATIN_A_UNSUPPORTED;

  pub const CURRENCY_SUPPORTED: u32 = 1; // euro 0x20ac
  pub const NOT_DEF: u32 = 1;
}

const BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_FIRST: &str = r##"!"#$%&'()*+,-./"##;
const BASIC_LATIN_DIGITS: &str = "0123456789";
const BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_SECOND: &str = ":;<=>?@";
const BASIC_LATIN_UPPERCASE_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_THIRD: &str = r"[\]^_`";
const BASIC_LATIN_LOWERCASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_FOURTH: &str = "{|}~";

const BASIC_LATIN: &str = concat!(
  BASIC_LATIN_UPPERCASE_ALPHABET,
  BASIC_LATIN_LOWERCASE_ALPHABET,
  BASIC_LATIN_DIGITS,
  BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_FIRST,
  BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_SECOND,
  BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_THIRD,
  BASIC_LATIN_PUNCTUATION_AND_SYMBOLS_FOURTH,
);

// euro 0x20ac snuck in as a supplement
//   originally part of the currency unicode block
const LATIN_SUPPLEMENT_PUNCTUATION_AND_SYMBOLS: &str = "¡¢£€¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿";
const LATIN_SUPPLEMENT_LETTERS_FIRST: &str = "ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ";
const LATIN_SUPPLEMENT_MATHEMATICAL_OPERATORS_FIRST: &str = "×";
const LATIN_SUPPLEMENT_LETTERS_SECOND: &str = "ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö";
const LATIN_SUPPLEMENT_MATHEMATICAL_OPERATORS_SECOND: &str = "÷";
const LATIN_SUPPLEMENT_LETTERS_THIRD: &str = "øùúûüýþÿ";

const LATIN_SUPPLEMENT: &str = concat!(
  LATIN_SUPPLEMENT_PUNCTUATION_AND_SYMBOLS,
  LATIN_SUPPLEMENT_LETTERS_FIRST,
  LATIN_SUPPLEMENT_MATHEMATICAL_OPERATORS_FIRST,
  LATIN_SUPPLEMENT_LETTERS_SECOND,
  LATIN_SUPPLEMENT_MATHEMATICAL_OPERATORS_SECOND,
  LATIN_SUPPLEMENT_LETTERS_THIRD
);

const LATIN_A: &str = "ĀāĂăĄąĆćĈĉĊċČčĎďĐđĒēĔĕĖėĘęĚěĜĝĞğĠġĢģĤĥĦħĨĩĪīĬĭĮįİıĲĳĴĵĶķĸĹĺĻļĽľĿŀŁłŃńŅņŇňŉŊŋŌōŎŏŐőŒœŔŕŖŗŘřŚśŜŝŞşŠšŢţŤťŦŧŨũŪūŬŭŮůŰűŲųŴŵŶŷŸŹźŻżŽž";

const SUPPORTED_GLYPHS: &str = concat!(BASIC_LATIN, LATIN_SUPPLEMENT, LATIN_A);

#[derive(Debug, Eq)]
pub struct Glyph {
  pub character: UnicodeChar,
  pub bbox: Bbox,
  pub contours: Vec<Vec<Point>>,
  pub pixels: Vec<Point>,
}

impl Glyph {
  pub fn new(character: UnicodeChar, contours: Vec<Vec<Point>>, pixels: Vec<Point>) -> Self {
    Self {
      character,
      bbox: Glyph::create_bbox(&contours),
      contours,
      pixels,
    }
  }

  pub fn glyphs() -> Vec<UnicodeChar> {
    let mut glyphs: Vec<_> = SUPPORTED_GLYPHS.chars().map(UnicodeChar::Char).collect();
    glyphs.push(UnicodeChar::NotDef);
    glyphs.push(UnicodeChar::UNICODE_SPACE);
    glyphs.push(UnicodeChar::UNICODE_NBSP);

    glyphs
  }

  pub fn scale_data(&mut self, scale: i16) {
    self
      .contours
      .iter_mut()
      .for_each(|contour| contour.iter_mut().for_each(|point| point.scale(scale)));
    self.bbox = Bbox {
      x_min: self.bbox.x_min * scale,
      y_min: self.bbox.y_min * scale,
      x_max: self.bbox.x_max * scale,
      y_max: self.bbox.y_max * scale,
    }
  }

  fn create_bbox(data: &Vec<Vec<Point>>) -> Bbox {
    let mut x_min = 0i16;
    let mut y_min = 0i16;
    let mut x_max = 0i16;
    let mut y_max = 0i16;

    for contour in data {
      for Point { x, y } in contour {
        let x = *x;
        let y = *y;

        if x < x_min {
          x_min = x;
        }
        if x > x_max {
          x_max = x;
        }

        if y < y_min {
          y_min = y;
        }
        if y > y_max {
          y_max = y;
        }
      }
    }

    Bbox {
      x_min,
      y_min,
      x_max,
      y_max,
    }
  }
}

impl From<&Glyph> for SimpleGlyph {
  fn from(val: &Glyph) -> Self {
    let contours = val
      .contours
      .iter()
      .map(|contour| {
        contour
          .iter()
          .map(|point| CurvePoint::on_curve(point.x, point.y))
          .collect::<Vec<_>>()
          .into()
      })
      .collect();

    SimpleGlyph {
      bbox: val.bbox,
      contours,
      instructions: Vec::new(),
    }
  }
}

impl PartialEq for Glyph {
  fn eq(&self, other: &Self) -> bool {
    self.character == other.character
  }
}

impl PartialOrd for Glyph {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Glyph {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.character.cmp(&other.character)
  }
}

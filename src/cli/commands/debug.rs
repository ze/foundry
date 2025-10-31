use std::path::PathBuf;

use anyhow::{Ok, Result};
use read_fonts::{FontRef, TableProvider};

pub fn debug(path: PathBuf) -> Result<()> {
  let bytes = std::fs::read(path)?;
  let font = FontRef::new(&bytes)?;

  let name = font.name()?;
  println!("name:");
  let name_data = name.string_data();
  for record in name.name_record() {
    println!(
      "  {}, {}, {}, {} = {}",
      record.platform_id(),
      record.encoding_id(),
      record.language_id(),
      record.name_id(),
      record.string(name_data)?
    );
  }

  let head = font.head()?;
  println!(
    "head:
  version: {}
  font_revision: {}
  checksum_adjustment: {}
  magic_number: {}
  flags: {:b}
  units_per_em: {}
  created: {}
  modified: {}
  x_min: {}
  y_min: {}
  x_max: {}
  y_max: {}
  mac_style: {:b}
  lowest_rec_ppem: {}
  font_direction_hint: {}
  index_to_loc_format: {}
  glyph_data_format: {}",
    head.version(),
    head.font_revision(),
    head.checksum_adjustment(),
    head.magic_number(),
    head.flags().bits(),
    head.units_per_em(),
    head.created().as_secs(),
    head.modified().as_secs(),
    head.x_min(),
    head.y_min(),
    head.x_max(),
    head.y_max(),
    head.mac_style().bits(),
    head.lowest_rec_ppem(),
    head.font_direction_hint(),
    head.index_to_loc_format(),
    head.glyph_data_format()
  );

  let hhea = font.hhea()?;
  println!(
    "hhea:
  version: {}
  ascender: {}
  descender: {}
  line_gap: {}
  advance_width_max: {}
  min_left_side_bearing: {}
  min_right_side_bearing: {}
  x_max_extent: {}
  caret_slope_rise: {}
  caret_slope_run: {}
  caret_offset: {}
  metric_data_format: {}
  number_of_h_metrics: {}",
    hhea.version(),
    hhea.ascender(),
    hhea.descender(),
    hhea.line_gap(),
    hhea.advance_width_max(),
    hhea.min_left_side_bearing(),
    hhea.min_right_side_bearing(),
    hhea.x_max_extent(),
    hhea.caret_slope_rise(),
    hhea.caret_slope_run(),
    hhea.caret_offset(),
    hhea.metric_data_format(),
    hhea.number_of_h_metrics()
  );

  let _maxp = font.maxp()?;
  println!("maxp:");

  let _os2 = font.os2()?;
  println!("os2:");

  let _post = font.post()?;
  println!("post:");

  let _cmap = font.cmap()?;
  println!("cmap:");

  let _glyf = font.glyf()?;
  println!("glyf:");

  let _loca = font.loca(None)?;
  println!("loca:");

  Ok(())
}

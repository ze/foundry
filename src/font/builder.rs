use std::{
  cmp::{max, min},
  collections::HashMap,
  vec,
};

use anyhow::Result;
use multimap::MultiMap;
use read_fonts::{
  tables::{cmap::PlatformId, layout::LookupFlag, os2::SelectionFlags},
  types::{GlyphId16, Tag},
};
use time::UtcDateTime;
use write_fonts::{
  FontBuilder, OffsetMarker,
  tables::{
    cmap::{Cmap, Cmap4, CmapSubtable, EncodingRecord},
    glyf::{Bbox, Glyf, GlyfLocaBuilder, SimpleGlyph},
    gpos::{
      Gpos, PairPos, PairPosFormat1, PairSet, PairValueRecord, PositionLookup, PositionLookupList,
      ValueRecord,
    },
    head::{Flags, Head, MacStyle},
    hhea::Hhea,
    hmtx::Hmtx,
    layout::{
      CoverageFormat1, CoverageTable, Feature, FeatureList, FeatureRecord, LangSys, Lookup, Script,
      ScriptList, ScriptRecord,
    },
    loca::{Loca, LocaFormat},
    maxp::Maxp,
    name::{Name, NameRecord},
    os2::Os2,
    post::Post,
    vmtx::LongMetric,
  },
  types::{BoundingBox, FWord, Fixed, LongDateTime, NameId, Version16Dot16},
};

use crate::font::{
  config::Config, dimensions::Dimensions, glyphs::Glyph, metadata::Metadata,
  unicode_char::UnicodeChar,
};

const UNITS_PER_EM: u16 = 2048;
const GRID_SIZE: u16 = 16;

pub struct Builder {
  glyphs: Vec<Glyph>,
  kerning: HashMap<(char, char), i16>,
}

#[allow(
  clippy::cast_possible_truncation,
  clippy::cast_possible_wrap,
  clippy::cast_sign_loss
)]
impl Builder {
  pub fn new(glyphs: Vec<Glyph>, kerning: HashMap<(char, char), i16>) -> Self {
    Self { glyphs, kerning }
  }

  pub fn build(&mut self, config: &Config) -> Result<Vec<u8>> {
    let scale_factor: i16 = (UNITS_PER_EM / GRID_SIZE) as i16;
    let one_unit = scale_factor;
    self
      .glyphs
      .iter_mut()
      .for_each(|glyph| glyph.scale_data(scale_factor));
    self
      .kerning
      .iter_mut()
      .for_each(|(_, value)| *value *= one_unit);

    let bounding_box = self.glyphs_rect();

    let mut builder = FontBuilder::new();

    let (glyf, loca, loca_format) = self.glyf_loca()?;

    let hmtx = self.hmtx(config.dimensions(), one_unit);

    let head = Builder::head(bounding_box, loca_format);

    let hhea = Builder::hhea(bounding_box, &hmtx, one_unit);

    let maxp = self.maxp();

    let (cmap, first_code, last_code) = self.cmap();

    let os2 = self.os2(config.metadata(), first_code, last_code, &hhea);

    let gpos = self.gpos();

    let name = Builder::name(config.metadata());
    let post = Builder::post();

    builder.add_table(&head)?;
    builder.add_table(&hhea)?;
    builder.add_table(&maxp)?;
    builder.add_table(&os2)?;
    builder.add_table(&hmtx)?;
    builder.add_table(&cmap)?;
    builder.add_table(&loca)?;
    builder.add_table(&glyf)?;
    builder.add_table(&gpos)?;
    builder.add_table(&name)?;
    builder.add_table(&post)?;

    let bytes = builder.build();
    Ok(bytes)
  }

  fn head(bounding_box: BoundingBox<i16>, loca_format: LocaFormat) -> Head {
    const FONT_REVISION: Fixed = Fixed::from_i32(1);
    const CHECKSUM_ADJUSTMENT: u32 = 0;
    const FLAGS: Flags = Flags::BASELINE_AT_Y_0.union(Flags::LSB_AT_X_0);
    const MAC_TIMESTAMP_OFFSET: i64 = 2_082_844_800;
    const MAC_STYLE: MacStyle = MacStyle::empty();
    const LOWEST_REC_PPEM: u16 = 8;
    let now = UtcDateTime::now().unix_timestamp() + MAC_TIMESTAMP_OFFSET;
    let now = LongDateTime::new(now);

    let loca_format = match loca_format {
      LocaFormat::Short => 0,
      LocaFormat::Long => 1,
    };

    Head::new(
      FONT_REVISION,
      CHECKSUM_ADJUSTMENT,
      FLAGS,
      UNITS_PER_EM,
      now,
      now,
      bounding_box.x_min,
      bounding_box.y_min,
      bounding_box.x_max,
      bounding_box.y_max,
      MAC_STYLE,
      LOWEST_REC_PPEM,
      loca_format,
    )
  }

  fn hhea(bounding_box: BoundingBox<i16>, hmtx: &Hmtx, one_unit: i16) -> Hhea {
    const LINE_GAP: FWord = FWord::new(0);

    const CARET_SLOPE_RISE: i16 = 1;
    const CARET_SLOPE_RUN: i16 = 0;
    const CARET_OFFSET: i16 = 0;

    let advance_width_max = hmtx
      .h_metrics
      .iter()
      .map(|metric| metric.advance)
      .max()
      .unwrap();
    let min_left_side_bearing = hmtx
      .h_metrics
      .iter()
      .map(|metric| metric.side_bearing)
      .min()
      .unwrap();

    let min_right_side_bearing = one_unit;
    let x_max_extent = bounding_box.x_max;
    let number_of_h_metrics: u16 = hmtx.h_metrics.len() as u16;

    Hhea::new(
      bounding_box.y_max.into(),
      bounding_box.y_min.into(),
      LINE_GAP,
      advance_width_max.into(),
      min_left_side_bearing.into(),
      min_right_side_bearing.into(),
      x_max_extent.into(),
      CARET_SLOPE_RISE,
      CARET_SLOPE_RUN,
      CARET_OFFSET,
      number_of_h_metrics,
    )
  }

  fn maxp(&self) -> Maxp {
    const NON_COMPOSITE: u16 = 0;
    const MAX_ZONES: u16 = 2;
    const NO_TWILIGHT_POINTS: u16 = 0;
    // TODO: these values are likely suboptimal
    const ONE_STORAGE_AREA: u16 = 1;
    const ONE_FUNCTION_DEF: u16 = 1;
    const NO_INSTRUCTION_DEF: u16 = 0;
    const STACK_ELEMENTS: u16 = 64;
    const NO_INSTRUCTIONS: u16 = 0;

    let num_glyphs: u16 = self.glyphs.len() as u16;
    let max_points = self
      .glyphs
      .iter()
      .map(|g| g.contours.iter().map(Vec::len).sum::<usize>() as u16)
      .max();
    let max_contours = self.glyphs.iter().map(|g| g.contours.len() as u16).max();
    Maxp {
      num_glyphs,
      max_points,
      max_contours,
      max_composite_points: Some(NON_COMPOSITE),
      max_composite_contours: Some(NON_COMPOSITE),
      max_zones: Some(MAX_ZONES),
      max_twilight_points: Some(NO_TWILIGHT_POINTS),
      max_storage: Some(ONE_STORAGE_AREA),
      max_function_defs: Some(ONE_FUNCTION_DEF),
      max_instruction_defs: Some(NO_INSTRUCTION_DEF),
      max_stack_elements: Some(STACK_ELEMENTS),
      max_size_of_instructions: Some(NO_INSTRUCTIONS),
      max_component_elements: Some(NON_COMPOSITE),
      max_component_depth: Some(NON_COMPOSITE),
    }
  }

  #[allow(clippy::too_many_lines)]
  fn os2(&self, metadata: &Metadata, first_code: u16, last_code: u16, hhea: &Hhea) -> Os2 {
    const MEDIUM_WIDTH_CLASS: u16 = 5;
    const FS_TYPE_INSTALLABLE_EMBEDDING: u16 = 0;
    const S_FAMILY_CLASS_NO_CLASSIFICATION: i16 = 0;
    const UNICODE_RANGE_1: u32 = 0b0111;
    const UNICODE_RANGE_2: u32 = 0b0010;
    const UNICODE_RANGE_3_AND_4: u32 = 0;
    const ACH_VEND_ID: Tag = Tag::new(b"FDRY");
    const SELECTION_FLAGS: SelectionFlags = SelectionFlags::REGULAR;
    const S_TYPO_LINE_GAP: i16 = 0;

    const PANOSE_FAMILY_KIND: u8 = 2;
    const PANOSE_SERIF_STYLE: u8 = 0;
    const PANOSE_EVEN_WIDTH: u8 = 3;
    const PANOSE_NO_CONTRAST: u8 = 0;
    const PANOSE_NO_STROKE_VARIATION: u8 = 0;
    const PANOSE_NO_ARM_STYLE: u8 = 0;
    const PANOSE_NO_LETTERFORM: u8 = 0;
    const PANOSE_NO_MIDLINE: u8 = 0;
    const PANOSE_NO_X_HEIGHT: u8 = 0;

    const PAGE_1: u32 = {
      const WINDOWS_1252_LATIN_1: u32 = 1 << 0;
      const WINDOWS_1250_LATIN_2: u32 = 1 << 1;
      const WINDOWS_1254_TURKISH: u32 = 1 << 4;
      const WINDOWS_1257_BALTIC: u32 = 1 << 7;
      WINDOWS_1252_LATIN_1 | WINDOWS_1250_LATIN_2 | WINDOWS_1254_TURKISH | WINDOWS_1257_BALTIC
    };
    const PAGE_2_UNSUPPORTED: u32 = 0;
    const UL_CODE_PAGE_RANGE: (u32, u32) = (PAGE_1, PAGE_2_UNSUPPORTED);

    const LOW_X_CHAR: UnicodeChar = UnicodeChar::Char('x');

    const NOTDEF_GLYPH_ID: u16 = 0;
    const US_DEFAULT_CHAR: Option<u16> = Some(NOTDEF_GLYPH_ID);

    const SPACE_UNICODE: u16 = 0x20;
    const US_BREAK_CHAR: Option<u16> = Some(SPACE_UNICODE);

    const PAIR_KERNING: u16 = 2;
    const US_MAX_CONTEXT: Option<u16> = Some(PAIR_KERNING);

    const NOT_OPTICAL: Option<u16> = None;

    let upm = f32::from(UNITS_PER_EM);

    let num_glyphs: i32 = self.glyphs.len() as i32;
    let x_avg_char_width: i16 = (self
      .glyphs
      .iter()
      .map(|glyph| i32::from(glyph.bbox.x_max) - i32::from(glyph.bbox.x_min))
      .sum::<i32>()
      / num_glyphs) as i16;

    let sx_height = self
      .glyphs
      .iter()
      .find(|g| g.character == LOW_X_CHAR)
      .map(|g| g.bbox.y_max);

    let ascender: i16 = hhea.ascender.into();
    let descender: i16 = hhea.descender.into();

    Os2 {
      x_avg_char_width,
      us_weight_class: metadata.font_subfamily().weight_class(),
      us_width_class: MEDIUM_WIDTH_CLASS,
      fs_type: FS_TYPE_INSTALLABLE_EMBEDDING,
      y_subscript_x_size: (0.65 * upm) as i16,
      y_subscript_y_size: (0.6 * upm) as i16,
      y_subscript_x_offset: 0,
      y_subscript_y_offset: (0.14 * upm) as i16,
      y_superscript_x_size: (0.65 * upm) as i16,
      y_superscript_y_size: (0.6 * upm) as i16,
      y_superscript_x_offset: 0,
      y_superscript_y_offset: (0.35 * upm) as i16,
      y_strikeout_size: (upm / 20.0) as i16,
      y_strikeout_position: (0.35 * upm) as i16,
      s_family_class: S_FAMILY_CLASS_NO_CLASSIFICATION,
      panose_10: [
        PANOSE_FAMILY_KIND,
        PANOSE_SERIF_STYLE,
        metadata.font_subfamily().panose_weight(),
        PANOSE_EVEN_WIDTH,
        PANOSE_NO_CONTRAST,
        PANOSE_NO_STROKE_VARIATION,
        PANOSE_NO_ARM_STYLE,
        PANOSE_NO_LETTERFORM,
        PANOSE_NO_MIDLINE,
        PANOSE_NO_X_HEIGHT,
      ],
      ul_unicode_range_1: UNICODE_RANGE_1,
      ul_unicode_range_2: UNICODE_RANGE_2,
      ul_unicode_range_3: UNICODE_RANGE_3_AND_4,
      ul_unicode_range_4: UNICODE_RANGE_3_AND_4,
      ach_vend_id: ACH_VEND_ID,
      fs_selection: SELECTION_FLAGS,
      us_first_char_index: first_code,
      us_last_char_index: last_code,
      s_typo_ascender: ascender,
      s_typo_descender: descender,
      s_typo_line_gap: S_TYPO_LINE_GAP,
      us_win_ascent: ascender.unsigned_abs(),
      us_win_descent: descender.unsigned_abs(),
      ul_code_page_range_1: Some(UL_CODE_PAGE_RANGE.0),
      ul_code_page_range_2: Some(UL_CODE_PAGE_RANGE.1),
      sx_height,
      s_cap_height: Some(ascender),
      us_default_char: US_DEFAULT_CHAR,
      us_break_char: US_BREAK_CHAR,
      us_max_context: US_MAX_CONTEXT,
      us_lower_optical_point_size: NOT_OPTICAL,
      us_upper_optical_point_size: NOT_OPTICAL,
    }
  }

  fn hmtx(&self, dimensions: &Dimensions, one_unit: i16) -> Hmtx {
    let h_metrics = self
      .glyphs
      .iter()
      .map(|glyph| {
        let advance: u16 = if glyph.character.is_space() {
          let space_width = dimensions.space_width();
          space_width * (one_unit as u16)
        } else {
          (glyph.bbox.x_max + one_unit) as u16
        };

        LongMetric::new(advance, glyph.bbox.x_min)
      })
      .collect();
    Hmtx::new(h_metrics, Vec::new())
  }

  fn cmap(&self) -> (Cmap, u16, u16) {
    const UNICODE_ENCODING_ID: u16 = 3;
    const WINDOWS_ENCODING_ID: u16 = 1;
    const CMAP_4_LANGUAGE: u16 = 0;
    const GLYPH_ID_ARRAY: Vec<u16> = Vec::new();

    let (start_code, end_code, id_delta, id_range_offsets) = self.cmap_segments();
    let first_code = start_code[0];
    let last_code = end_code[end_code.len() - 1];

    let cmap4 = Cmap4::new(
      CMAP_4_LANGUAGE,
      end_code,
      start_code,
      id_delta,
      id_range_offsets,
      GLYPH_ID_ARRAY,
    );
    let subtable = CmapSubtable::Format4(cmap4);

    let encoding_records = vec![
      EncodingRecord::new(PlatformId::Unicode, UNICODE_ENCODING_ID, subtable.clone()),
      EncodingRecord::new(PlatformId::Windows, WINDOWS_ENCODING_ID, subtable),
    ];
    let cmap = Cmap::new(encoding_records);
    (cmap, first_code, last_code)
  }

  fn glyf_loca(&self) -> Result<(Glyf, Loca, LocaFormat)> {
    let mut builder = GlyfLocaBuilder::new();

    for glyph in &self.glyphs {
      let simple: SimpleGlyph = glyph.into();
      builder.add_glyph(&simple)?;
    }

    Ok(builder.build())
  }

  fn gpos(&self) -> Gpos {
    const DFLT_TAG: Tag = Tag::new(b"DFLT");
    const LATN_TAG: Tag = Tag::new(b"latn");
    const KERN_TAG: Tag = Tag::new(b"kern");

    let lang_sys = LangSys::new(vec![0]);
    let script = Script::new(Some(lang_sys), Vec::new());
    let dflt_record = ScriptRecord::new(DFLT_TAG, script.clone());
    let latn_record = ScriptRecord::new(LATN_TAG, script);
    let script_list = ScriptList::new(vec![dflt_record, latn_record]);

    let feature = Feature::new(None, vec![0]);
    let feature_record = FeatureRecord::new(KERN_TAG, feature);
    let feature_list = FeatureList::new(vec![feature_record]);

    let char_to_gid: HashMap<char, u16> = self
      .glyphs
      .iter()
      .enumerate()
      .filter_map(|(id, g)| match g.character {
        UnicodeChar::NotDef => None,
        UnicodeChar::Char(c) => Some((c, id as u16)),
      })
      .collect();

    let grouped_kerning: MultiMap<u16, (u16, i16)> = self
      .kerning
      .iter()
      .map(|((left, right), value)| {
        (
          *char_to_gid.get(left).unwrap(),
          (*char_to_gid.get(right).unwrap(), *value),
        )
      })
      .collect();

    let left_glyphs = {
      let mut keys: Vec<u16> = grouped_kerning.keys().copied().collect();
      keys.sort_unstable();
      keys
    };

    let glyph_array: Vec<GlyphId16> = left_glyphs.iter().copied().map(GlyphId16::new).collect();
    let coverage = CoverageFormat1::new(glyph_array);
    let coverage = CoverageTable::Format1(coverage);

    let pair_sets: Vec<PairSet> = left_glyphs
      .iter()
      .map(|left| {
        let mut pairs = grouped_kerning.get_vec(left).cloned().unwrap();
        pairs.sort_by_key(|(right, _)| *right);

        let records: Vec<PairValueRecord> = pairs
          .into_iter()
          .map(|(right, value)| {
            PairValueRecord::new(
              right.into(),
              ValueRecord::new().with_x_advance(value),
              ValueRecord::default(),
            )
          })
          .collect();
        PairSet::new(records)
      })
      .collect();
    let pair_pos = PairPosFormat1::new(coverage, pair_sets);
    let pair_pos = PairPos::Format1(pair_pos);
    let lookup = Lookup::new(LookupFlag::IGNORE_MARKS, vec![pair_pos]);
    let lookup = PositionLookup::Pair(lookup);
    let lookup_list = PositionLookupList::new(vec![lookup]);

    Gpos::new(script_list, feature_list, lookup_list)
  }

  fn name(metadata: &Metadata) -> Name {
    let font_name = metadata.font_name();
    let font_subfamily = metadata.font_subfamily();
    let font_subfamily = font_subfamily.name();
    let version = metadata.version();

    let mut records: Vec<NameRecord> = [
      (NameId::COPYRIGHT_NOTICE, metadata.copyright()),
      (NameId::FAMILY_NAME, font_name),
      (NameId::SUBFAMILY_NAME, font_subfamily),
      (
        NameId::UNIQUE_ID,
        &format!("{font_name};{font_subfamily};v{version}"),
      ),
      (NameId::FULL_NAME, &format!("{font_name} {font_subfamily}")),
      (NameId::VERSION_STRING, &format!("Version {version}")),
      (
        NameId::POSTSCRIPT_NAME,
        &format!("{font_name}-{font_subfamily}"),
      ),
    ]
    .iter()
    .flat_map(|(name_id, value)| Builder::name_record(*name_id, value))
    .collect();
    records.sort();

    Name::new(records)
  }

  fn post() -> Post {
    const ITALIC_ANGLE: Fixed = Fixed::from_i32(0);

    const UNDERLINE_POSITION: FWord = FWord::new(-75);
    const UNDERLINE_THICKNESS: FWord = FWord::new(50);

    const IS_MONOSPACED: bool = false;
    const IS_FIXED_PITCH: u32 = if IS_MONOSPACED { 1 } else { 0 };
    const UNUSED: u32 = 0;

    Post {
      version: Version16Dot16::VERSION_3_0,
      italic_angle: ITALIC_ANGLE,
      underline_position: UNDERLINE_POSITION,
      underline_thickness: UNDERLINE_THICKNESS,
      is_fixed_pitch: IS_FIXED_PITCH,
      min_mem_type42: UNUSED,
      max_mem_type42: UNUSED,
      min_mem_type1: UNUSED,
      max_mem_type1: UNUSED,
      num_glyphs: None,
      glyph_name_index: None,
      string_data: None,
    }
  }

  fn cmap_segments(&self) -> (Vec<u16>, Vec<u16>, Vec<i16>, Vec<u16>) {
    const FINAL_START_CODE: u16 = 0xFFFF;
    const FINAL_END_CODE: u16 = 0xFFFF;
    const FINAL_ID_DELTA: i16 = 1;
    const FINAL_ID_RANGE_OFFSETS: u16 = 0;

    let mut start_codes: Vec<u16> = Vec::new();
    let mut end_codes: Vec<u16> = Vec::new();
    let mut id_deltas: Vec<i16> = Vec::new();
    let mut id_range_offsets: Vec<u16> = Vec::new();

    let mut started = false;
    let mut start_code: u16 = 0;
    let mut end_code: u16 = 0;
    let mut id_delta: i16 = 0;

    let mut iter = self.glyphs.iter().enumerate();
    let next = iter.next();
    debug_assert!(next.is_some_and(|(_, n)| n.character == UnicodeChar::NotDef));
    for (gid, g) in iter {
      let c = match g.character {
        UnicodeChar::NotDef => unreachable!(),
        UnicodeChar::Char(c) => c,
      };
      let unicode = c as u16;
      let gid = gid as i16;
      let delta = gid - (unicode as i16);

      if !started {
        start_code = unicode;
        end_code = unicode;
        id_delta = delta;
        started = true;
      } else if unicode == end_code + 1 {
        end_code = unicode;
      } else {
        start_codes.push(start_code);
        end_codes.push(end_code);
        id_deltas.push(id_delta);
        id_range_offsets.push(0);
        start_code = unicode;
        end_code = unicode;
        id_delta = delta;
      }
    }

    if started {
      start_codes.push(start_code);
      end_codes.push(end_code);
      id_deltas.push(id_delta);
      id_range_offsets.push(0);
    }

    start_codes.push(FINAL_START_CODE);
    end_codes.push(FINAL_END_CODE);
    id_deltas.push(FINAL_ID_DELTA);
    id_range_offsets.push(FINAL_ID_RANGE_OFFSETS);
    (start_codes, end_codes, id_deltas, id_range_offsets)
  }

  fn name_record(name_id: NameId, value: &str) -> Vec<NameRecord> {
    const UNICODE_PLATFORM_ID: u16 = 0;
    const UNICODE_ENCODING_ID: u16 = 0;
    const UNICODE_LANGUAGE_ID: u16 = 0;

    const WINDOWS_PLATFORM_ID: u16 = 3;
    const WINDOWS_ENCODING_ID: u16 = 1;
    const WINDOWS_LANGUAGE_ID_EN_US: u16 = 0x0409;

    let marker = OffsetMarker::new(value.to_string());
    let unicode_record = NameRecord::new(
      UNICODE_PLATFORM_ID,
      UNICODE_ENCODING_ID,
      UNICODE_LANGUAGE_ID,
      name_id,
      marker.clone(),
    );
    let windows_record = NameRecord::new(
      WINDOWS_PLATFORM_ID,
      WINDOWS_ENCODING_ID,
      WINDOWS_LANGUAGE_ID_EN_US,
      name_id,
      marker,
    );
    vec![unicode_record, windows_record]
  }

  fn glyphs_rect(&self) -> BoundingBox<i16> {
    let mut total_x_min = 0i16;
    let mut total_y_min = 0i16;
    let mut total_x_max = 0i16;
    let mut total_y_max = 0i16;
    for glyph in &self.glyphs {
      let Bbox {
        x_min,
        y_min,
        x_max,
        y_max,
      } = glyph.bbox;

      total_x_min = min(total_x_min, x_min);
      total_y_min = min(total_y_min, y_min);
      total_x_max = max(total_x_max, x_max);
      total_y_max = max(total_y_max, y_max);
    }

    BoundingBox {
      x_min: total_x_min,
      y_min: total_y_min,
      x_max: total_x_max,
      y_max: total_y_max,
    }
  }
}

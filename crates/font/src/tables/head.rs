use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub struct HeadTable {
  #[allow(dead_code)]
  pub version: f32,
  #[allow(dead_code)]
  pub font_revision: f32,
  #[allow(dead_code)]
  pub checksum_adjustment: u32,
  #[allow(dead_code)]
  pub magic_number: u32,
  #[allow(dead_code)]
  pub flags: u16,
  #[allow(dead_code)]
  pub units_per_em: u16,
  #[allow(dead_code)]
  pub created: i64,
  #[allow(dead_code)]
  pub modified: i64,
  #[allow(dead_code)]
  pub x_min: i16,
  #[allow(dead_code)]
  pub y_min: i16,
  #[allow(dead_code)]
  pub x_max: i16,
  #[allow(dead_code)]
  pub y_max: i16,
  #[allow(dead_code)]
  pub mac_style: u16,
  #[allow(dead_code)]
  pub lowest_rec_ppem: u16,
  #[allow(dead_code)]
  pub font_direction_hint: i16,
  #[allow(dead_code)]
  pub index_to_loc_format: i16,
  #[allow(dead_code)]
  pub glyph_data_format: i16,
}

// locaテーブルの構造
#[derive(Debug)]
#[allow(dead_code)]

pub struct LocaTable {
  #[allow(dead_code)]
  pub offsets: Vec<u32>,
}

// glyfテーブルの構造
#[derive(Debug)]
#[allow(dead_code)]
pub struct GlyfTable {
  #[allow(dead_code)]
  pub glyphs: Vec<Glyph>,
}

// glyphの構造 (単純化)
#[derive(Debug)]
#[allow(dead_code)]
pub struct Glyph {
  #[allow(dead_code)]
  pub number_of_contours: i16,
  #[allow(dead_code)]
  pub x_min: i16,
  #[allow(dead_code)]
  pub y_min: i16,
  #[allow(dead_code)]
  pub x_max: i16,
  #[allow(dead_code)]
  pub y_max: i16,
  #[allow(dead_code)]
  pub data: Vec<u8>, // グリフのデータ
}

impl HeadTable {
  pub fn parse(data: &[u8]) -> Result<Self, String> {
    let mut cursor = Cursor::new(data);
    
    Ok(HeadTable {
      version: cursor.read_f32::<BigEndian>().map_err(|e| e.to_string())?,
      font_revision: cursor.read_f32::<BigEndian>().map_err(|e| e.to_string())?,
      checksum_adjustment: cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?,
      magic_number: cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?,
      flags: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
      units_per_em: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
      created: cursor.read_i64::<BigEndian>().map_err(|e| e.to_string())?,
      modified: cursor.read_i64::<BigEndian>().map_err(|e| e.to_string())?,
      x_min: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      y_min: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      x_max: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      y_max: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      mac_style: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
      lowest_rec_ppem: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
      font_direction_hint: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      index_to_loc_format: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
      glyph_data_format: cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?,
    })
  }
}

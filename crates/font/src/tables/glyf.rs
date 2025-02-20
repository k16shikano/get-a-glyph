use crate::truetype::{SimpleGlyph, GlyphType};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::tables::loca::{LocaTable, LocaFormat};

#[derive(Debug)]
pub struct GlyfTable {
  #[allow(dead_code)]
  pub glyphs: Vec<Glyph>,
}

#[derive(Debug)]
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
  pub data: Vec<u8>,
}

impl GlyfTable {
  pub fn parse(data: &[u8], loca_table: &LocaTable) -> Result<Self, String> {
    let mut glyphs = Vec::new();
    let mut cursor = Cursor::new(data);
    
    for i in 0..(loca_table.offsets.len() - 1) {
      let offset = match loca_table.format {
        LocaFormat::Offset16 => loca_table.offsets[i] * 2, // Offset16の場合、2倍する
        LocaFormat::Offset32 => loca_table.offsets[i],     // Offset32の場合
      };
      
      let next_offset = match loca_table.format {
        LocaFormat::Offset16 => loca_table.offsets[i + 1] * 2,
        LocaFormat::Offset32 => loca_table.offsets[i + 1],
      };
      
      let offset_usize = offset as usize;
      let next_offset_usize = next_offset as usize;
      
      let current_position = cursor.position();
      cursor.seek(SeekFrom::Start(offset as u64)).map_err(|e| e.to_string())?;
      
      let glyph_size = next_offset_usize - offset_usize;
      
      let mut glyph = Glyph {
        number_of_contours: 0,
        x_min: 0,
        y_min: 0,
        x_max: 0,
        y_max: 0,
        data: Vec::new(),
      };
      if glyph_size == 0 {
        // サイズ0のグリフの場合、number_of_contoursを0または-1に設定
        glyph.number_of_contours = 0; // または -1
        glyph.x_min = 0;
        glyph.y_min = 0;
        glyph.x_max = 0;
        glyph.y_max = 0;
        glyph.data = Vec::new();
      } else {
        // グリフヘッダーを読み込む
        glyph.number_of_contours = cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?;
        glyph.x_min = cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?;
        glyph.y_min = cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?;
        glyph.x_max = cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?;
        glyph.y_max = cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?;
        
        // アウトラインデータを読み込む
        let mut outline_data = vec![0u8; glyph_size - 10]; // ヘッダーサイズを考慮
        cursor.read_exact(&mut outline_data).map_err(|e| e.to_string())?;
        glyph.data = outline_data;
      }

      glyphs.push(glyph);
      cursor.seek(SeekFrom::Start(current_position)).map_err(|e| e.to_string())?;
    }
    
    Ok(GlyfTable { glyphs })
  }

  pub fn get_glyph_data(&self, glyph_id: usize) -> Result<SimpleGlyph, String> {
    if let Some(glyph) = self.glyphs.get(glyph_id) {
      match Glyph::get_glyph_type(glyph) {
        GlyphType::Simple => SimpleGlyph::parse(glyph),
        GlyphType::Composite => Err("Composite glyphs are not supported yet".to_string()),
        GlyphType::Empty => Ok(SimpleGlyph {
          end_pts_of_contours: vec![],
          instruction_length: 0,
          instructions: vec![],
          flags: vec![],
          x_coordinates: vec![],
          y_coordinates: vec![],
          points: vec![],
        }),
      }
    } else {
      Err(format!("Glyph not found: {}", glyph_id))
    }
  }
}

#[allow(dead_code)]
impl Glyph {
  fn get_glyph_type(glyph: &Glyph) -> GlyphType {
    if glyph.number_of_contours > 0 {
      GlyphType::Simple
    } else if glyph.number_of_contours < 0 {
      GlyphType::Composite
    } else {
      GlyphType::Empty
    }
  }
}
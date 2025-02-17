use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub struct LocaTable {
  #[allow(dead_code)]
  pub offsets: Vec<u32>,
  #[allow(dead_code)]
  pub format: LocaFormat,
}

// locaテーブルの形式を表現する列挙型
#[derive(Debug)]
pub enum LocaFormat {
  #[allow(dead_code)]
  Offset16,
  #[allow(dead_code)]
  Offset32,
}

impl LocaTable {
  pub fn parse(data: &[u8], num_glyphs: u16, index_to_loc_format: i16) -> Result<Self, String> {
    let mut offsets = Vec::new();
    let mut cursor = Cursor::new(data);
    
    // index_to_loc_format に基づいて LocaFormat を決定
    let format = match index_to_loc_format {
      0 => LocaFormat::Offset16,
      1 => LocaFormat::Offset32,
      _ => return Err("Invalid index_to_loc_format value".to_string()),
    };
    
    // オフセットを読み込む
    for _ in 0..(num_glyphs + 1) {
      match format {
        LocaFormat::Offset16 => {
          let offset = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
          offsets.push(offset as u32); // u32 に変換して格納
        }
        LocaFormat::Offset32 => {
          let offset = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
          offsets.push(offset);
        }
      }
    }
    
    Ok(LocaTable { offsets, format })
  }

  pub fn get_glyph_offset(loca_table: &[u8], glyph_id: u16, index_to_loc_format: i16) -> u32 {
    match index_to_loc_format {
      0 => {
        // shortオフセット形式
        let offset = (loca_table[glyph_id as usize * 2] as u32) << 8 | loca_table[glyph_id as usize * 2 + 1] as u32;
        offset * 2
      }
      1 => {
        // longオフセット形式
        (loca_table[glyph_id as usize * 4] as u32) << 24 |
        (loca_table[glyph_id as usize * 4 + 1] as u32) << 16 |
        (loca_table[glyph_id as usize * 4 + 2] as u32) << 8 |
        loca_table[glyph_id as usize * 4 + 3] as u32
      }
      _ => {
        // 不正なオフセット形式
        println!("Error: Invalid indexToLocFormat");
        0 // エラーを示す値を返す
      }
    }
  }
}


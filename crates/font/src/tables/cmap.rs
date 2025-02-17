use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct CmapTable {
  #[allow(dead_code)]
  pub version: u16,
  #[allow(dead_code)]
  pub num_tables: u16,
  #[allow(dead_code)]
  pub encoding_records: Vec<EncodingRecord>,
}

#[derive(Debug, Copy, Clone)]
pub struct EncodingRecord {
  #[allow(dead_code)]
  pub platform_id: u16,
  #[allow(dead_code)]
  pub encoding_id: u16,
  #[allow(dead_code)]
  pub offset: u32,
}

#[derive(Debug)]
pub struct CmapFormat4 {
  #[allow(dead_code)]
  format: u16,
  #[allow(dead_code)]
  length: u16,
  #[allow(dead_code)]
  language: u16,
  #[allow(dead_code)]
  seg_count_x2: u16,
  #[allow(dead_code)]
  search_range: u16,
  #[allow(dead_code)]
  entry_selector: u16,
  #[allow(dead_code)]
  range_shift: u16,
  #[allow(dead_code)]
  end_code: Vec<u16>,
  #[allow(dead_code)]
  reserved_pad: u16,
  #[allow(dead_code)]
  start_code: Vec<u16>,
  #[allow(dead_code)]
  id_delta: Vec<i16>,
  #[allow(dead_code)]
  id_range_offset: Vec<u16>,
  #[allow(dead_code)]
  glyph_id_array: Vec<u16>,
}

impl CmapTable {
  pub fn parse(data: &[u8]) -> Result<Self, String> {
    let mut cursor = Cursor::new(data);
    let version = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let num_tables = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let mut encoding_records = Vec::new();
    
    for _ in 0..num_tables {
      let platform_id = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
      let encoding_id = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
      let offset = cursor.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
      
      encoding_records.push(EncodingRecord {
        platform_id,
        encoding_id,
        offset,
      });
    }
    
    Ok(CmapTable {
      version,
      num_tables,
      encoding_records,
    })
  }
  
  pub fn parse_format4(data: &[u8], subtable_length: u16) -> Result<CmapFormat4, String> {
    let mut cursor = Cursor::new(data);
    
    let format = 4; // cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let length = subtable_length; // cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let language = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let seg_count_x2 = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let search_range = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let entry_selector = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    let range_shift = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    
    let seg_count = seg_count_x2 / 2;
    
    let mut end_code = Vec::new();
    for _ in 0..seg_count {
      end_code.push(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?);
    }
    
    let reserved_pad = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    
    let mut start_code = Vec::new();
    for _ in 0..seg_count {
      start_code.push(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?);
    }
    
    let mut id_delta = Vec::new();
    for _ in 0..seg_count {
      id_delta.push(cursor.read_i16::<BigEndian>().map_err(|e| e.to_string())?);
    }
    
    let mut id_range_offset = Vec::new();
    for _ in 0..seg_count {
      id_range_offset.push(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?);
    }
    
    let glyph_id_array_length = (length as usize - 16 - (8 * seg_count as usize)) / 2;
    let mut glyph_id_array = Vec::new();
    for _ in 0..glyph_id_array_length {
      glyph_id_array.push(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?);
    }
    
    let cmap_format4 = CmapFormat4 {
      format,
      length,
      language,
      seg_count_x2,
      search_range,
      entry_selector,
      range_shift,
      end_code,
      reserved_pad,
      start_code,
      id_delta,
      id_range_offset,
      glyph_id_array,
    };
    
    Ok(cmap_format4)
  }

pub fn get_glyph_id(char_code: u16, cmap_format4: &CmapFormat4) -> Option<u16> {
  let seg_count = cmap_format4.seg_count_x2 / 2;
  
  for i in 0..seg_count as usize {
    if char_code >= cmap_format4.start_code[i] && char_code <= cmap_format4.end_code[i] {
      if cmap_format4.id_range_offset[i] == 0 {
        return Some(((char_code as i32 + cmap_format4.id_delta[i] as i32) % 65536) as u16);
      } else {
        let index = (cmap_format4.id_range_offset[i] as usize) / 2
        + (char_code - cmap_format4.start_code[i]) as usize
        + i;
        
        if index < cmap_format4.glyph_id_array.len() {
          return Some(cmap_format4.glyph_id_array[index]);
        } else {
          return None;
          }
        }
      }
    }
    None // 該当するセグメントが見つからない
  }
} 



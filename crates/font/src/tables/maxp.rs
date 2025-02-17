use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
pub struct MaxpTable {
  #[allow(dead_code)]
  pub version: f32,
  #[allow(dead_code)]
  pub num_glyphs: u16,
  // バージョン0.5より大きい場合は以下もある
  #[allow(dead_code)]
  pub max_points: Option<u16>,
  #[allow(dead_code)]
  pub max_contours: Option<u16>,
  #[allow(dead_code)]
  pub max_component_points: Option<u16>,
  #[allow(dead_code)]
  pub max_component_contours: Option<u16>,
  #[allow(dead_code)]
  pub max_zones: Option<u16>,
  #[allow(dead_code)]
  pub max_twilight_points: Option<u16>,
  #[allow(dead_code)]
  pub max_storage: Option<u16>,
  #[allow(dead_code)]
  pub max_function_defs: Option<u16>,
  #[allow(dead_code)]
  pub max_instruction_defs: Option<u16>,
  #[allow(dead_code)]
  pub max_stack_elements: Option<u16>,
  #[allow(dead_code)]
  pub max_size_of_instructions: Option<u16>,
  #[allow(dead_code)]
  pub max_component_elements: Option<u16>,
  #[allow(dead_code)]
  pub max_component_depth: Option<u16>,
}

impl MaxpTable {
  pub fn parse(data: &[u8]) -> Result<Self, String> {
    let mut cursor = Cursor::new(data);
    
    let version = cursor.read_f32::<BigEndian>().map_err(|e| e.to_string())?;
    let num_glyphs = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
    
    // バージョンが1.0の場合、追加のフィールドを読み込む
    let (max_points, max_contours, max_component_points, max_component_contours,
      max_zones, max_twilight_points, max_storage, max_function_defs,
      max_instruction_defs, max_stack_elements, max_size_of_instructions,
      max_component_elements, max_component_depth) = if version >= 1.0 {
        (
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
          Some(cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?),
        )
      } else {
        (None, None, None, None, None, None, None, None, None, None, None, None, None)
      };
      
      Ok(MaxpTable {
        version,
        num_glyphs,
        max_points,
        max_contours,
        max_component_points,
        max_component_contours,
        max_zones,
        max_twilight_points,
        max_storage,
        max_function_defs,
        max_instruction_defs,
        max_stack_elements,
        max_size_of_instructions,
        max_component_elements,
        max_component_depth,
      })
    }
  }
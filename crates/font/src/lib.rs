mod tables;

use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
use tables::cmap::CmapTable;
use tables::glyf::GlyfTable;
use tables::loca::LocaTable;
use tables::head::HeadTable;
use tables::maxp::MaxpTable;
pub trait Parse {
  fn from_bytes(data: &[u8]) -> Result<Self, String> where Self: Sized;
}

#[derive(Debug)]
pub struct Sfnt {
  #[allow(dead_code)]
  version: u32,
  #[allow(dead_code)]
  tables: u16,
  #[allow(dead_code)]
  search_range: u16,
  #[allow(dead_code)]
  entry_selector: u16,
  #[allow(dead_code)]
  range_shift: u16,
  #[allow(dead_code)]
  records: Vec<TableRecord>,
}

#[derive(Debug)]
pub struct TableRecord {
  #[allow(dead_code)]
  tag: String,
  #[allow(dead_code)]
  checksum: u32,
  #[allow(dead_code)]
  offset: u32,
  #[allow(dead_code)]
  length: u32,
}

impl Parse for Sfnt {
  fn from_bytes(data: &[u8]) -> Result<Sfnt, String> {
    let mut cursor = Cursor::new(data);
    let sfnt_version: u32 = cursor.read_u32::<BigEndian>().unwrap();
    let num_tables: u16 = cursor.read_u16::<BigEndian>().unwrap();
    let search_range: u16 = cursor.read_u16::<BigEndian>().unwrap();
    let entry_selector: u16 = cursor.read_u16::<BigEndian>().unwrap();
    let range_shift: u16 = cursor.read_u16::<BigEndian>().unwrap();

    let mut tables = Vec::new();
        
    for _ in 0..num_tables as usize {
      let mut tag_bytes = [0u8; 4];
      cursor.read_exact(&mut tag_bytes).unwrap();
      let checksum = cursor.read_u32::<BigEndian>().unwrap();
      let offset = cursor.read_u32::<BigEndian>().unwrap();
      let length = cursor.read_u32::<BigEndian>().unwrap();
      let tag = String::from_utf8_lossy(&tag_bytes).to_string();

      tables.push(TableRecord { tag, checksum, offset, length });
    }

    Ok(Sfnt { 
      version: sfnt_version,
      tables: num_tables,
      search_range,
      entry_selector,
      range_shift,
      records: tables 
      }) 
  }   
}

impl Sfnt {
  pub fn get_glyph_data(&self, _glyph_name: &str, data: &[u8]) -> Result<String, String> {
    let mut cursor = Cursor::new(data);
    let saved_position = cursor.position();

    let Some(head_table) = self.records.iter().find(|record| record.tag == "head") else {
      return Err("headテーブルが見つかりません".to_string());
    };
    let Some(loca_table) = self.records.iter().find(|record| record.tag == "loca") else {
      return Err("locaテーブルが見つかりません".to_string());
    };
    let Some(glyf_table) = self.records.iter().find(|record| record.tag == "glyf") else {
      return Err("glyfテーブルが見つかりません".to_string());
    };
    let Some(cmap_table) = self.records.iter().find(|record| record.tag == "cmap") else {
      return Err("cmapテーブルが見つかりません".to_string());
    };
    let Some(maxp_table) = self.records.iter().find(|record| record.tag == "maxp") else {
      return Err("maxpテーブルが見つかりません".to_string());
    };
    
    let mut cmap_data = vec![0; cmap_table.length as usize];
    cursor.set_position(cmap_table.offset as u64);
    cursor.read_exact(&mut cmap_data).unwrap();
    
    let cmap_table = CmapTable::parse(&cmap_data)?;
    let platform_id = 0; // 固定
    let encoding_id = 3; // 固定
    
    let mut cmap_cursor = Cursor::new(&cmap_data);
    let Some(record) = cmap_table.encoding_records.iter().find(|&record| record.platform_id == platform_id && record.encoding_id == encoding_id) else {
      return Err("指定されたEncodingRecordが見つかりません".to_string());
    };
    
    let subtable_offset = record.offset;
    cmap_cursor.set_position(subtable_offset as u64);
    let _format = cmap_cursor.read_u16::<BigEndian>().unwrap() as u32;
    let subtable_length = cmap_cursor.read_u16::<BigEndian>().unwrap() as u32;
    let mut subtable_data = vec![0; (subtable_length - subtable_offset + 16) as usize]; // 16はfomatとlengthフィールドの分。これらはcursorで読み込み済みなのでparserでも読み込むとsubtableのデータが不足する
    cmap_cursor.read_exact(&mut subtable_data).unwrap();
    let subtable = CmapTable::parse_format4(&subtable_data, subtable_length as u16)?;
    
    let glyph_id = CmapTable::get_glyph_id(0x0041, &subtable).unwrap_or(0);

    cursor.set_position(saved_position);
    let mut head_data = vec![0; head_table.length as usize];
    cursor.set_position(head_table.offset as u64);
    cursor.read_exact(&mut head_data).unwrap();

    let head_table = HeadTable::parse(&head_data)?;
    let index_to_loc_format = head_table.index_to_loc_format;

    cursor.set_position(saved_position);
    let mut maxp_data = vec![0; maxp_table.length as usize];
    cursor.set_position(maxp_table.offset as u64);
    cursor.read_exact(&mut maxp_data).unwrap();

    let maxp_table = MaxpTable::parse(&maxp_data)?;
    let num_glyphs = maxp_table.num_glyphs;
    
    cursor.set_position(saved_position);
    let mut loca_data = vec![0; loca_table.length as usize];
    cursor.set_position(loca_table.offset as u64);
    cursor.read_exact(&mut loca_data).unwrap();

    let loca_table = LocaTable::parse(&loca_data, num_glyphs, index_to_loc_format)?;

    cursor.set_position(saved_position);
    let mut glyf_data = vec![0; glyf_table.length as usize];
    cursor.set_position(glyf_table.offset as u64);
    cursor.read_exact(&mut glyf_data).unwrap();
    
    let _glyf_table = GlyfTable::parse(&glyf_data, &loca_table)?;
    let _glyph_offset = LocaTable::get_glyph_offset(&loca_data, glyph_id, index_to_loc_format);
    
    Ok(format!("{:?}", glyf_data))
  }
}

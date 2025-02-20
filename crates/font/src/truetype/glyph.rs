use crate::tables::glyf::Glyph;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt as _};
use crate::truetype::outline::Point;

#[derive(Debug)]
pub enum GlyphType {
  Simple,
  Composite,
  Empty,
}

#[derive(Debug)]
pub struct SimpleGlyph {
  pub end_pts_of_contours: Vec<u16>,
  pub instruction_length: u16,
  pub instructions: Vec<u8>,
  pub flags: Vec<u8>,
  pub x_coordinates: Vec<i16>,
  pub y_coordinates: Vec<i16>,
  pub points: Vec<Point>,
}

#[derive(Debug)]
pub struct CompositeGlyph {
  pub components: Vec<Component>,
}

#[derive(Debug)]
pub struct Component {
  pub glyph_index: u16,
  pub transform: Option<Transform>,
}

#[derive(Debug)]
pub struct Transform {
  pub a: f32,
  pub b: f32,
  pub c: f32,
  pub d: f32,
  pub x: f32,
  pub y: f32,
}

// 定数を追加
const ON_CURVE_POINT: u8 = 0x01;
const X_SHORT_VECTOR: u8 = 0x02;
const Y_SHORT_VECTOR: u8 = 0x04;
const REPEAT_FLAG: u8 = 0x08;
const X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR: u8 = 0x10;
const Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR: u8 = 0x20;

impl SimpleGlyph {
  pub fn parse(glyph: &Glyph) -> Result<Self, String> {
    let mut reader = Cursor::new(&glyph.data);
    let number_of_contours = glyph.number_of_contours;
    
    let mut end_pts_of_contours = Vec::new();
    for _ in 0..number_of_contours {
      end_pts_of_contours.push(reader.read_u16::<BigEndian>().unwrap());
    }
    
    let instruction_length = reader.read_u16::<BigEndian>().unwrap();
    let mut instructions = Vec::new();
    for _ in 0..instruction_length {
      instructions.push(reader.read_u8().unwrap());
    }
    
    // フラグと座標の解析
    let num_points = if !end_pts_of_contours.is_empty() {
      *end_pts_of_contours.last().unwrap() as usize + 1
    } else {
      0
    };
    
    let mut flags = Vec::with_capacity(num_points);
    let mut x_coordinates = Vec::with_capacity(num_points);
    let mut y_coordinates = Vec::with_capacity(num_points);
    
    while flags.len() < num_points {
      let flag = reader.read_u8().unwrap();
      flags.push(flag);
      if (flag & REPEAT_FLAG) != 0 {
        let repeat_count = reader.read_u8().unwrap() as usize;
        for _ in 0..repeat_count {
          flags.push(flag);
        }
      }
    }
    
    // X座標の読み込み
    let mut x = 0;
    for i in 0..num_points {
      let flag = flags[i];
      let value: i16;
      if (flag & X_SHORT_VECTOR) != 0 {
        // X_SHORT_VECTORがセットされている場合
        if (flag & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR) != 0 {
          // X_IS_SAME_OR_POSITIVE_X_SHORT_VECTORがセットされている場合、正の値
          value = reader.read_u8().unwrap() as i16;
        } else {
          // X_IS_SAME_OR_POSITIVE_X_SHORT_VECTORがセットされていない場合、負の値
          value = -(reader.read_u8().unwrap() as i16);
        }
      } else {
        // X_SHORT_VECTORがセットされていない場合
        if (flag & X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR) != 0 {
          // X_IS_SAME_OR_POSITIVE_X_SHORT_VECTORがセットされている場合、前の値と同じ
          value = 0; // 実際には前の値を使用するが、ここでは0として扱う
        } else {
          // X_IS_SAME_OR_POSITIVE_X_SHORT_VECTORがセットされていない場合、2バイト
          value = reader.read_i16::<BigEndian>().unwrap();
        }
      }
      x += value;
      x_coordinates.push(x);
    }
    
    // Y座標の読み込み
    let mut y = 0;
    for i in 0..num_points {
      let flag = flags[i];
      let value: i16;
      if (flag & Y_SHORT_VECTOR) != 0 {
        // Y_SHORT_VECTORがセットされている場合
        if (flag & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR) != 0 {
          // Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTORがセットされている場合、正の値
          value = reader.read_u8().unwrap() as i16;
        } else {
          // Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTORがセットされていない場合、負の値
          value = -(reader.read_u8().unwrap() as i16);
        }
      } else {
        // Y_SHORT_VECTORがセットされていない場合
        if (flag & Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR) != 0 {
          // Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTORがセットされている場合、前の値と同じ
          value = 0; // 実際には前の値を使用するが、ここでは0として扱う
        } else {
          // Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTORがセットされていない場合、2バイト
          value = reader.read_i16::<BigEndian>().unwrap();
        }
      }
      y += value;
      y_coordinates.push(y);
    }
    
    let mut points = Vec::new();
    for i in 0..num_points {
      points.push(Point {
        x: x_coordinates[i],
        y: y_coordinates[i],
        on_curve: (flags[i] & ON_CURVE_POINT) != 0
      });
    }
    
    Ok(SimpleGlyph {
      end_pts_of_contours,
      instruction_length,
      instructions,
      flags,
      x_coordinates,
      y_coordinates,
      points,
    })
  }
}




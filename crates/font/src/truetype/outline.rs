use crate::truetype::SimpleGlyph;

pub struct Outline {
  pub contours: Vec<Contour>,
}

pub struct Contour {
  pub points: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: i16,
  pub y: i16,
  pub on_curve: bool,
}

impl Outline {
  pub fn from_simple_glyph(glyph_data: &SimpleGlyph) -> Self {
    let mut contours = Vec::new();
    let mut start = 0;
    
    for &end in &glyph_data.end_pts_of_contours {
      let points = glyph_data.points[start..=end as usize].to_vec();
      contours.push(Contour { points });
      start = end as usize + 1;
    }
    
    Outline { contours }
  }
}

fn points_to_svg_path(glyph: &SimpleGlyph) -> String {
  let mut path_data = String::new();
  let mut current_point: Option<Point> = None;
  let mut contour_start_index = 0;

  for contour_end_index in &glyph.end_pts_of_contours {
      let mut i = contour_start_index;
      while i <= *contour_end_index as usize {
          let p = glyph.points[i];
          if current_point.is_none() {
              // Move to the first point of the contour
              path_data.push_str(&format!("M{} {} ", p.x, p.y));
              current_point = Some(p);
              i += 1;
              continue;
          }

          match (glyph.points.get(i), glyph.points.get(i + 1), glyph.points.get(i + 2)) {
              (Some(&p1), Some(&p2), Some(&p3)) if !p1.on_curve && !p2.on_curve && p3.on_curve => {
                  // Two off-curve points followed by an on-curve point, use a cubic Bezier curve
                  // let last_point = current_point.unwrap();
                  path_data.push_str(&format!("C {} {}, {} {}, {} {} ", p1.x, p1.y, p2.x, p2.y, p3.x, p3.y));
                  current_point = Some(p3);
                  i += 3;
              }
              (Some(&p1), Some(&p2), _) if !p1.on_curve && p2.on_curve => {
                  // One off-curve point, use a quadratic Bezier curve
                  //let last_point = current_point.unwrap();
                  path_data.push_str(&format!("Q {} {}, {} {} ", p1.x, p1.y, p2.x, p2.y));
                  current_point = Some(p2);
                  i += 2;
              }
              (Some(&p1), _, _) if p1.on_curve => {
                  // Line
                  path_data.push_str(&format!("L {} {} ", p1.x, p1.y));
                  current_point = Some(p1);
                  i += 1;
              }
              _ => {
                  i += 1;
              }
          }
      }
      // Close the path for the current contour
      path_data.push_str("Z");
      current_point = None; // Reset for the next contour
      contour_start_index = *contour_end_index as usize + 1;
  }

  return path_data;
} 

pub fn simple_glyph_to_svg(glyph: &SimpleGlyph) -> String {
  let path_data = points_to_svg_path(glyph);

  format!(
      "<svg viewBox='0 0 1468 1468' xmlns='http://www.w3.org/2000/svg'><path d='{}' /></svg>",
      path_data
  )
}
#[derive(Clone)]
pub struct Luma<T: Clone> {
  pub data: [T; 1]
}

pub struct ImageBuffer<T> {
  width: u32,
  height: u32,
  data: Vec<T>
}

impl ImageBuffer<Luma<u8>> {
  pub fn new(width: u32, height: u32) -> Self {
    ImageBuffer {
      width,
      height,
      data: vec![Luma { data: [255] }; (width * height) as usize]
    }
  }
  
  pub fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut Luma<u8>)> + '_ {
    struct PixelIterator<'a> {
      width: u32,
      height: u32,
      data: &'a mut [Luma<u8>],
      x: u32,
      y: u32,
    }
    
    impl<'a> Iterator for PixelIterator<'a> {
      type Item = (u32, u32, &'a mut Luma<u8>);
      
      fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
          return None;
        }
        
        let x = self.x;
        let y = self.y;
        let idx = (y * self.width + x) as usize;
        
        // 次の位置を計算
        self.x += 1;
        if self.x >= self.width {
          self.x = 0;
          self.y += 1;
        }
        
        // unsafe: データスライスから1要素だけ取り出す
        let pixel = unsafe {
          &mut *(&mut self.data[idx] as *mut Luma<u8>)
        };
        
        Some((x, y, pixel))
      }
    }
    
    let data = unsafe {
      std::slice::from_raw_parts_mut(
        self.data.as_mut_ptr() as *mut Luma<u8>,
        self.data.len()
      )
    };
    
    PixelIterator {
      width: self.width,
      height: self.height,
      data,
      x: 0,
      y: 0,
    }
  }
  
  pub fn save(&self, _path: &str) -> Result<(), String> {
    // 画像を保存する実装
    todo!("画像保存機能の実装")
  }
} 
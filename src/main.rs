extern crate font;
extern crate image;
extern crate ps_parser;

use font::Sfnt;
use image::{Luma, ImageBuffer};
use ps_parser::PostScriptInterpreter;

fn main() {
    // Type 1フォントデータを引数で受け取る
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("フォントデータのパスを引数として指定してください");
        std::process::exit(1);
    }
    let font_path = &args[1];
    let font_data = std::fs::read(font_path).expect("フォントファイルの読み込みに失敗しました");
    let font: Sfnt = font::Parse::from_bytes(&font_data).expect("Type 1フォントのパースに失敗しました");

    // グリフのPostScriptデータを取得
    let glyph_name = "A"; // 例として"A"グリフを使用
    let glyph_data = font.get_glyph_data(glyph_name, &font_data).expect("グリフデータの取得に失敗しました");

    println!("{:?}", glyph_data);
    
    // PostScriptデータを処理
    let mut interpreter = PostScriptInterpreter::new();
    let glyph_outline = interpreter.interpret(&glyph_data).expect("PostScriptデータの処理に失敗しました");

    // 簡易シェイピングとラスタリング
    let (width, height) = (100, 100); // 例として100x100の画像サイズを使用
    let mut image = ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let value = if glyph_outline.contains(x as f32, y as f32) { 0 } else { 255 };
        *pixel = Luma{ data: [value] };
    }

    // 画像を保存
    image.save("output.png").expect("画像の保存に失敗しました");
}

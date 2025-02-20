extern crate font;
extern crate rasterizer;

use font::Sfnt;
use font::truetype::outline::simple_glyph_to_svg;
//use rasterizer::{Luma, ImageBuffer};

fn main() {
    // フォントファイルを引数で受け取る
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("フォントファイルのパスを引数として指定してください");
        std::process::exit(1);
    }
    let font_path = &args[1];
    let font_data = std::fs::read(font_path).expect("フォントファイルの読み込みに失敗しました");
    let font: Sfnt = font::Parse::from_bytes(&font_data).expect("Type 1フォントのパースに失敗しました");

    // グリフのアウトラインデータを取得
    if args.len() < 3 {
        eprintln!("グリフ名を引数として指定してください");
        std::process::exit(1);
    }
    let glyph_name = &args[2];
    let glyph_data = font.get_glyph_data(glyph_name, &font_data).expect("グリフデータの取得に失敗しました");

    let svg_path = simple_glyph_to_svg(&glyph_data);
    println!("{}", svg_path);

    // TrueTypeフォントの場合

}

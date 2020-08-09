extern crate qrcode_generator;

use std::io::Read;
use std::fs;

fn main() {
  use qrcode_generator::QrCodeEcc;
  let need = String::from("links.txt");

  let contents = fs::read_to_string(need)
    .expect("Something went wrong reading the file");

  for path in contents.lines() {
    println!("{}", path);
    qrcode_generator::to_svg_to_file(format!("http://ждуавтобус.рф/{}/", path), QrCodeEcc::Low, 256, None::<&str>, format!("svg/{}.svg", path)).unwrap();
  }
}

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// カスタムエラー型を定義します
#[derive(Debug)]
enum FileOpError {
    // エラーの種類をここに追加します
    ReadError(String),
    WriteError(String),
}

// Resultのエイリアスを定義します
type Result<T> = std::result::Result<T, FileOpError>;

// file 読み込みサンプル
// fn main() -> std::io::Result<()> {
//     let mut file = File::open("foo.txt")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     assert_eq!(contents, "Hello, world!");
//     Ok(())
// }

fn read_file(path: &str) -> Result<String> {
    // ファイル読み込みのロジックをここに実装します
    println!("read");
    // Ok("read");
    let mut file: File = File::open(path).map_err({|e| FileOpError::ReadError(e.to_string())}).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("contents: {}", contents);
    return Ok(contents);
}

// 書き込みサンプル
// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }

// fn write_file(path: &str, content: &str) -> Result<()> {
fn write_file(path: &str, content: &str){
    // ファイル書き込みのロジックをここに実装します
    println!("write");

    let mut file: File = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    println!("write to: {}", path)
}

pub fn main() {
    println!("ファイルコピープログラム");
    println!("入力ファイル名を入力してください:");

    // ユーザー入力の処理とファイル操作のロジックをここに実装します
    let mut from = String::new();
    std::io::stdin().read_line(&mut from).unwrap();
    let from = from.trim();

    println!("出力ファイル名を入力してください:");

    let mut to = String::new();
    std::io::stdin().read_line(&mut to).unwrap();
    let to = to.trim();

    println!("copy file from: {from}, to: {to}", from = from, to = to);

    let content = read_file(from).unwrap();
    write_file(to, content.as_str());
}
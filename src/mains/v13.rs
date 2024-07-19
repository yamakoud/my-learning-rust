use std::fs;
use std::io::{self, Read, Write};
// use std::env;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("読み取りに失敗しました");
    input.trim().to_string()
}

pub fn main() -> io::Result<()> {
    let mode = loop {
        let input = get_input("モード選択(encrypt/decrypt):");
        if input == "encrypt" || input == "decrypt" {
            break input;
        } else {
            println!("not matched");
        }
    };

    let input_file = get_input("インプットファイルの場所:");
    let output_file = get_input("アウトプットファイルの場所:");

    let mut content = String::new();
    fs::File::open(input_file)?.read_to_string(&mut content)?;

    let result = match mode.as_str() {
        "encrypt" => encrypt(&content),
        "decrypt" => decrypt(&content),
        _ => {
            println!("無効なモードです。'encrypt' または 'decrypt' を使用してください。");
            return Ok(());
        }
    };

    fs::write(&output_file, result)?;
    println!("処理が完了しました。結果は {} に保存されました。", output_file);

    Ok(())
}

fn encrypt(text: &str) -> String {
    // TODO: 暗号化ロジックを実装
    let result: String;
    result = text.to_string();  // この行は適切な実装に置き換えてください
    return result;
}

fn decrypt(text: &str) -> String {
    // TODO: 復号化ロジックを実装
    let result: String;
    result = text.to_string();  // この行は適切な実装に置き換えてください
    return result;
}

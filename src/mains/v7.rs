use std::io;

pub fn main() {
    println!("温度変換プログラム");
    println!("1: 摂氏から華氏");
    println!("2: 華氏から摂氏");

    let mut mode = String::new();

    println!("type ");
    let result = std::io::stdin().read_line(&mut mode);
    match result {
        Ok(_) => println!("mode: {}", mode),
        Err(error) => println!("error: {}", error),
    }

    let mode = mode.trim();

    println!("温度を入力してください");
    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("有効な数値を入力してください");
            return;
        }
    };

    println!("入力された温度: {}", temperature);

    let ret: f64;
    if mode == "1" {
        ret = celsius_to_fahrenheit(temperature);
    }else{
        ret = fahrenheit_to_celsius(temperature);
    }

    println!("結果: {}", ret.to_string())
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return (celsius * 9.0 / 5.0) + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
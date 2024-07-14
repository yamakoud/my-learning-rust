use std::f64::consts::PI;
use std::io;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    // ここにフィールドを追加
    diameter: f64,
}

struct Rectangle {
    // ここにフィールドを追加
    height: f64,
    width: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        // ここに円の面積計算ロジックを実装
        self.diameter * 3.14
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        // ここに長方形の面積計算ロジックを実装
        self.height * self.width
    }
}

pub fn main() {
    println!("図形の面積計算プログラム");
    println!("1: 円");
    println!("2: 長方形");

    // ここにユーザー入力の処理と面積計算のロジックを実装
    let mut mode = String::new();
    std::io::stdin().read_line(&mut mode).unwrap();

    let mode = mode.trim();

    if mode == "1" {
        // 1 circle
        println!("diameter");
        let mut diameter = String::new();
        std::io::stdin().read_line(&mut diameter).unwrap();
        let diameter: f64 = diameter.trim().parse().unwrap();

        // call circle
        let circle = Circle {
            diameter: diameter,
        };
        let area = circle.area();
        println!("area: {}", area.to_string())

    }else if mode == "2"{
        // 2 rectangle
        println!("width");
        let mut width = String::new();
        std::io::stdin().read_line(&mut width).unwrap();
        let width: f64 = width.trim().parse().unwrap();
        println!("height");
        let mut height = String::new();
        std::io::stdin().read_line(&mut height).unwrap();
        let height: f64 = height.trim().parse().unwrap();

        // call rectangle
        let rectangle = Rectangle{
            height: height,
            width: width,
        };

        let area = rectangle.area();
        println!("area: {}", area.to_string());
    }else{
        panic!("err")
    }
}
trait Area {
    fn area(&self) -> u32;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect area: {}", rect.area());
}

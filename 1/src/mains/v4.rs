trait Area {
    fn area(&self) -> u32;
    fn can_hold(&self, other: &Rectangle) -> bool;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 60,
        height: 100,
    };

    println!("rect1 area: {}", rect1.area());
    println!("rect2 area: {}", rect2.area());

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}

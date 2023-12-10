trait Executable {
    fn execute(&self);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write { text: String },
    ChangeColor { red: u8, green: u8, blue: u8 },
}

impl Executable for Message {
    fn execute(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write { text } => println!("Write {}", text),
            Message::ChangeColor { red, green, blue } => {
                println!("Change color to ({}, {}, {})", red, green, blue)
            }
        }
    }
}

pub fn main() {
    let messages: Vec<Message> = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write {
            text: String::from("Hello, world!"),
        },
        Message::ChangeColor {
            red: 255,
            green: 0,
            blue: 0,
        },
    ];

    for message in messages {
        message.execute();
    }
}

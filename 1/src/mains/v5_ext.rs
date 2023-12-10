trait Executable {
    fn execute(&self) -> Result<String, &'static str>;
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write { text: String },
    ChangeColor { red: u8, green: u8, blue: u8 },
    Other,
}

impl Executable for Message {
    fn execute(&self) -> Result<String, &'static str> {
        match self {
            Message::Quit => Ok("Quit".to_string()),
            Message::Move { x, y } => Ok(format!("Move to ({}, {})", x, y)),
            Message::Write { text } => Ok(format!("Write {}", text)),
            Message::ChangeColor { red, green, blue } => {
                Ok(format!("Change color to ({}, {}, {})", red, green, blue))
            }
            _ => Err("Not implemented"),
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
        Message::Other,
    ];

    for message in messages {
        match message.execute() {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{}", error),
        }
    }
}

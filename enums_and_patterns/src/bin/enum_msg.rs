#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        match self {
            // method body would be defined here
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("{}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to ({}, {}, {})", r, g, b)
            }
        }
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

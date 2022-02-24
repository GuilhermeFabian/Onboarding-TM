#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(u8, u8, u8),
}

impl Message {
  fn call(&self) {
    dbg!(self);
  }
}

fn main() {
  let m = Message::ChangeColor(255, 255, 255);
  m.call();
}

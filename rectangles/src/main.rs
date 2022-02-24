#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// Instance Methods
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

// Static methods
impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  let rect2 = Rectangle {
    width: 20,
    height: 30,
  };

  let square = Rectangle::square(10);

  println!("The area of the rectangle is {} square pixels.", rect1.area());
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
  println!("Can rect2 hold square? {}", rect2.can_hold(&square));

  dbg!(rect1);
}

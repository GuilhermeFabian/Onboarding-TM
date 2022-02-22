fn main() {
  println!("Hello, world!");

  let five = plus_one(five());

  another_function(five, 'h');
}

fn another_function(value: i32, unit_label: char) {
  println!("The measurement is: {}{}.", value, unit_label);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

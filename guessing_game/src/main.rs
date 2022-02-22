use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  // In Rust, variables are immutable by default.
  // To make a variable mutable, we add mut before the variable name.
  let mut guess = String::new();

  io::stdin()
      // The & indicates that this argument is a reference.
      // References are immutable by default.
      // Hence, you need to write &mut guess rather than &guess to make it mutable.
      .read_line(&mut guess)
      .expect("Failed to read line");

  println!("You guessed: {}", guess); // The {} set of curly brackets is a placeholder.
}

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  // The kind of range expression we’re using here takes the form start..end.
  // It is inclusive on the lower bound but exclusive on the upper bound.
  // Alternatively, we could pass the range 1..=100.
  let secret_number = rand::thread_rng().gen_range(1..101);

  loop {
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

    // Rust allows us to shadow the previous value of guess with a new one.
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess); // The {} set of curly brackets is a placeholder.

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}

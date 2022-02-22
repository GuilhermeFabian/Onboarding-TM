fn main() {
  const CONDITION: bool = true;
  const NUMBER: i32 = if CONDITION { 5 } else { 6 };

  if NUMBER % 4 == 0 {
    println!("Number is divisible by 4.");
  } else if NUMBER % 3 == 0 {
    println!("Number is divisible by 3.");
  } else if NUMBER % 2 == 0 {
    println!("Number is divisible by 2.");
  } else {
    println!("Number is not divisible by 4, 3, or 2.");
  }
}

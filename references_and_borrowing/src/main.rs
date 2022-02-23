fn main() {
  let s1 = String::from("Hello!");
  let len = calculate_length(&s1);

  println!("The length of '{}' is {}", s1, len);
}

// A reference is like a pointer in that it’s an address
// we can follow to access data stored at that address that is owned by some other variable.
// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.
fn calculate_length(s: &String) -> usize {
  s.len()
}

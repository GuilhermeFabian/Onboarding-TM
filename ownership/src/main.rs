fn main() {
  let s1 = String::from("Hello!");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);

  let x = 5;
  // Primitive types can be copied without invalidating the variables from which it was copied.
  let y = x;

  println!("x = {}, y = {}", x, y);

  takes_ownership(s2); // s2's value moves into the function...
                                  // ... and so is no longer valid here

  makes_copy(y);      // x would move into the function,
                                  // but i32 is Copy, so it's okay to still
                                  // use x afterward
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

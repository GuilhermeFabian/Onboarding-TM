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

  let s3 = gives_ownership(); // gives_ownership moves its return
  let s4 = String::from("Hello");
  let s5 = takes_and_gives_back(s4); // s4 is moved into
                                                    // takes_and_gives_back, which also
                                                    // moves its return value into s5

  println!("s3: {}, s5: {}", s3, s5);

  let (s6, length) = calculate_length(s5);

  println!("s6: {}, length: {}", s6, length);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownership() -> String {
  let some_string = String::from("Yours");
  some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}

fn main() {
  let s1 = String::from("Hello!");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);

  let x = 5;
  // Primitive types can be copied without invalidating the variables from which it was copied.
  let y = x;

  println!("x = {}, y = {}", x, y);
}

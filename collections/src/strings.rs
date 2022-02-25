pub fn run_examples() {
  let mut s1 = String::from("foo");
  let s2 = "bar";

  s1.push_str(s2);

  println!("s2 is {}", s1);

  let s3 = &s1[..3];

  println!("s3 is {}.", s3);

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
}

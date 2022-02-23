fn main() {
  let s = String::from("Hello World!");
  let word = first_word(&s);

  println!("{}", word);

  let my_string = String::from("Hello World");

  let word = first_word(&my_string[0..6]);
  println!("{}", word);
  let word = first_word(&my_string[..]);
  println!("{}", word);
  let word = first_word(&my_string);
  println!("{}", word);

  let my_string_literal = "hello world";
  let word = first_word(&my_string_literal[0..6]);
  println!("{}", word);
  let word = first_word(&my_string_literal[..]);
  println!("{}", word);
  let word = first_word(my_string_literal);
  println!("{}", word);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}

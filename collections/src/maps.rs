use std::collections::HashMap;

pub fn run_examples() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores);

  {
    let inserted_value = scores.entry(String::from("Red")).or_insert(50);

    println!("inserted_value: {}", inserted_value);
  }

  println!("{:?}", scores);

  let text = "Green Yellow White Black";
  let mut last_value = 0;

  for word in text.split_whitespace() {
    let count = scores.entry(word.to_string()).or_insert(0);
    *count += 1 + &last_value;
    last_value += 1;
  }

  println!("{:?}", scores);
}

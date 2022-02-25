pub fn run_examples() {
  let mut v: Vec<i32> = Vec::new();

  let v2 = vec![1, 2, 3];

  for el in &v2 {
    println!("{}", el);
  }

  v.push(1);
  v.push(2);

  for el in &v {
    println!("{}", el);
  }

  let last: &i32 = &v2[v2.len() - 1];

  println!("Last: {}.", last);

  match v2.get(v2.len()) {
    Some(last) => println!("The last value is: {}.", last),
    None => (),
  }

  let v3 = vec![1, 2, 3, 4, 5];
  let first = &v3[0];

  // v3.push(6);

  println!("First: {}.", first);
}

fn main() {
  let x = 5;

  // We’re effectively creating a new variable when we use the let keyword again,
  // we can change the type of the value but reuse the same name.
  let x = x + 1;

  {
      let x = x * 2;
      println!("The value of x in the inner scope is: {}", x);
  }

  println!("The value of x is: {}", x);
}

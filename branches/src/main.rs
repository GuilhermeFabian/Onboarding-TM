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

  loop_example();
  loop_with_return();
}

fn loop_example() {
  let mut count = 0;

  'counting_up: loop {
    println!("Count = {}", count);

    let mut remaining = 10;

    loop {
      println!("remaining = {}", remaining);

      if remaining == 9 {
        break;
      }

      if count == 2 {
        break 'counting_up;
      }

      remaining -= 1;
    }

    count += 1;
  }

  println!("End count {}", count);
}

fn loop_with_return() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is: {}.", result);
}

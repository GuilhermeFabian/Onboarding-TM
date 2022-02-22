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
  while_example();
  for_index_example();
  for_each_example();
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

fn while_example() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFEOFF!!!");
}

fn for_index_example() {
  let list = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("The value is: {}.", list[index]);

    index += 1;
  }
}

fn for_each_example() {
  let list = [10, 20, 30, 40, 50];

  for element in list {
    println!("The value is: {}", element);
  }
}

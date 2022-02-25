use std::fs::File;
use std::io::{ self, ErrorKind, Read };

fn main() {
  let file_name = "hello.txt";
  let f = File::open(file_name);

  let _f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(file_name) {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file {:?}", e),
      },
      _ => panic!("Problem opening the file {:?}", error),
    },
  };

  let _f2 = File::open(file_name).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create(file_name).unwrap_or_else(|error| {
        panic!("Problem creating the file {:?}", error);
      })
    } else {
      panic!("Problem opening the file {:?}", error);
    }
  });

  let _f3 = File::open(file_name).expect(format!("Failed to open {}.", file_name).as_str());

  let username = match read_username_from_file() {
    Ok(username) => username,
    Err(_) => panic!("Errrror!"),
  };

  println!("Username: {}.", username);

  let username2 = match read_username_shorted_version() {
    Ok(username2) => username2,
    Err(_) => panic!("Errrror!"),
  };

  println!("Username2: {}.", username2.trim());

  println!("{}", last_char_of_first_line("line 1").unwrap());
  println!("{}", last_char_of_first_line("\nline 2").unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s.trim().to_string()),
    Err(e) => Err(e),
  }
}

fn read_username_shorted_version() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}

fn main() {
  let user = build_user(String::from("someone"), String::from("someone@example.com"));

  let x = Point(1, 1, 1);
  let white = Color(255, 255, 255);
  println!("Hello, {}", user.username);
}

fn build_user(username: String, email: String) -> User {
  User { active: true, username, email, sign_in_count: 1 }
}

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

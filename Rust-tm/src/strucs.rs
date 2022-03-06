// Struct - custom data type
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Strct
struct Birthday(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string()
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };
  c.green = 100;
  println!("Color: {} {} {}", c.red, c.green, c.blue);
  let mut me = Birthday(2, 14, 95);
  me.0 = 3;
  println!("Birth day: {} {} {}", me.0, me.1, me.2);

  let mut p = Person::new("John", "Doe");
  println!("Person: {}", p.full_name());
  p.set_last_name("Mary");
  println!("Person: {}", p.full_name());
  println!("Person: {:?}", p.to_tuple());
}

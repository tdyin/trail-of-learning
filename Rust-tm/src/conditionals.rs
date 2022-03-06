pub fn run() {
  let age: u8 = 18;
  let check_id: bool = false;
  let know_age = true;

  if age >= 21 && check_id || know_age {
    println!("Bartender: What you want to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: What are you doing here?");
  } else {
    println!("Bartender: Your ID?");
  }
  
  let is_of_age = if age >= 21 {true} else {false};
  println!("Is of age: {}", is_of_age);


}
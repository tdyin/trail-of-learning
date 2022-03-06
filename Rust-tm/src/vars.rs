pub fn run() {
  let name = "Tony";
  let mut age = 30;
  println!("My name is {}, and I an {}.", name, age);
  age = 31;
  println!("My name is {}, and I an {}.", name, age);

  const ID: i32 = 001;
  println!("ID: {}", ID);

  let (my_name, my_age) = ("Tony", 30);
  println!("{} is {}.", my_name, my_age)
}

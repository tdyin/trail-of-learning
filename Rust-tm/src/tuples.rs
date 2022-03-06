// max 12 elements (can different type)

pub fn run(){
  let person: (&str, &str, i8) = ("Tony", "China", 30);

  println!("{} is from {}, and {} is {}.", person.0, person.1, person.0, person.2)
}
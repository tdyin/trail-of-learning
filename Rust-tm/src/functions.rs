pub fn run() {
  greeting("Hello", "Moto");

  let get_sum = add(5, 5);
  println!("Sum: {}", get_sum);

  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // Clousure
  println!("C Sum: {}", add_nums(3, 3));
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2 // no semicolon means return
}

fn greeting(greet: &str, name: &str) {
  println!("{}, {}", greet, name);
}

pub fn run() {
  let hello_immutable = "Hello";
  // let mut hello_immutable = "Hello"; // Doesn't work
  let mut hello_growable = String::from("Hello ");

  hello_growable.push('w'); // for char
  hello_growable.push_str("orld!"); // for str

  println!("{}", hello_growable);
  println!("Length: {}", hello_immutable.len());

  println!("Capacity: {}", hello_growable.capacity());
  println!("Is empty: {}", hello_growable.is_empty());

  println!("Contains 'world': {}", hello_growable.contains("world"));

  println!("Repalce: {}", hello_growable.replace("world", "there"));

  for word in hello_growable.split_whitespace() {
    println!("{}", word);
  }

  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}", s);

  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}

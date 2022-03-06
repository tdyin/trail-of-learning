// fixed list, elements type same

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  numbers[2] = 20; // mut keywrod

  println!("{:?}", numbers);

  println!("Single value: {}", numbers[0]);
  println!("Length: {}", numbers.len());

  println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}

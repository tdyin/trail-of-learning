// resizeable array

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  numbers[2] = 20;

  numbers.push(6);
  numbers.push(7);

  numbers.pop();

  println!("{:?}", numbers);

  println!("Single value: {}", numbers[0]);

  println!("Length: {}", numbers.len());

  println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  for x in numbers.iter() {
    println!("Number: {}", x)
  }

  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Number: {:?}", numbers)  

}

/* *
 * Primitive
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (unsigned/singned)
 * Floats: f32, f64
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 */

pub fn run() {
  let x = 1; // i32
  let y = 2.5; // f64
  let z: i64 = 666666666;
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  let bo = true;

  let is_greater: bool = 10 < 5;

  let a1 = 'a'; // char

  let face = '\u{1F605}';

  println!("{:?}", (x, y, z, bo, is_greater, a1, face))
}

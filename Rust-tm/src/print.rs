pub fn run() {
  println!("Hello from the print.rs file");

  println!("{}", 1);

  println!(
    "{0} like {1}, and {0} like {2}.",
    "I", "music", "programming"
  );

  println!("{who} like to play {what}.", who = "I", what = "bass");

  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  println!("{:?}", (12, true, "hello"));

  println!("10 + 10 = {}", 10 + 10);
}

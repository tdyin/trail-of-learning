enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Moving up"),
    Movement::Down => println!("Moving down"),
    Movement::Left => println!("Moving left"),
    Movement::Right => println!("Moving right"),
  }
}

pub fn run() {
  let a1 = Movement::Left;
  let a2 = Movement::Up;
  let a3 = Movement::Right;
  let a4 = Movement::Down;

  move_avatar(a1);
  move_avatar(a2);
  move_avatar(a3);
  move_avatar(a4);
}

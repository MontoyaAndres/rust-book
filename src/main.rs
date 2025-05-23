fn get_value() -> i32 {
  let x = 5;
  let x = x + 1;
  x
}

fn main() {
  let value = get_value();
  let condition = if value > 5 {
    "greater than 5"
  } else {
    "less than or equal to 5"
  };

  let mut count = 0;
  'label: loop {
    count += 1;
    if count == 5 {
      break 'label;
    }
  }

  println!("The value is: {}", value);
  println!("The condition is: {}", condition);
  println!("The loop ran {} times", count);
}

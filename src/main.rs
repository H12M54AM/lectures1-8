use rand::prelude::*;

fn main() {
  let max = 100;
  let num_i = rand::thread_rng().gen::<i32>();
  let num_f = rand::thread_rng().gen::<f32>();
  let num = rand::thread_rng().gen_range(0..max);

  short(num_i, num_f, num);
  long(num_i, num_f, num);
}

fn short(num_i:i32, num_f:f32, num:i32) {
  println!("Max Number (Int)  : {}", num_i);
  println!("Max Number (Float): {}", num_f);
  println!("Random Number     : {}", num);
}

fn long(num_i:i64, num_f:f64, num:i64) {
  println!("Max Number (Int)  : {}", num_i);
  println!("Max Number (Float): {}", num_f);
  println!("Random Number     : {}", num);
}
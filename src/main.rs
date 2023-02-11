/**
 * Program Displays random variables from the
 * rand crate and utilizes two functions to
 * compare the long and short version of the
 * variable.
 *
 * Edward Naidoo
 * Feb 8, 2023
 * BCIT - Business Stats
 */
use rand::prelude::*;

fn main() {
  let max = 100;
  let num_i = rand::thread_rng().gen::<i32>();
  let num_f = rand::thread_rng().gen::<f32>();
  let num = rand::thread_rng().gen_range(0..max);

  short(num_i, num_f, num);
  println!("");
  long(num_i, num_f, num);
}

/**
 * Short Version
 * Outputs the values as a 32 Bit int and float
 * from randomized variable.
 */
fn short(num_i: i32, num_f: f32, num: i32) {
  println!("Following as a 32 bit value");
  println!("Max Number (Int)   : {}", num_i);
  println!("Max Number (Float) : {}", num_f);
  println!("Random Number      : {}", num);
}

/**
 * Long Version
 * Outputs the values as a 64 Bit int and float
 * from randomized variable.
 */
fn long(num_i: i32, num_f: f32, num: i32) {
  println!("Following as a 64 bit value");
  println!("Max Number (Int)   : {}", num_i as i64);
  println!("Max Number (Float) : {}", num_f as f64);
  println!("Random Number      : {}", num as i64);
}
use rand::prelude::*;

fn main() {
  let max = 100;
  let num_i = rand::thread_rng().gen::<i32>();
  let num_f = rand::thread_rng().gen::<f32>();
  let num = rand::thread_rng().gen_range(0..max);

  println!("Max Number (Int)  : {}", num_i);
  println!("Max Number (Float): {}", num_f);
  println!("Random Number     : {}", num);
}
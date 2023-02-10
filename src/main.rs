// mod Mathy;

mod gen_num;
use rand::prelude::*;
use stats::*;


fn main() {
  // let x = gen_num::Num{max: 10}
  let num = rand::thread_rng().gen::<i32>();
  // stats::mean();
  println!("Random Number: {}", num);
}
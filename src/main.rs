  #![feature(iter_next_chunk)]


use std::env;

mod advent;
mod day_1;
mod day_2;
mod day_3;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    day_3::step_1();
    day_3::step_2();
  } else {
    match args[1].as_str() {
      "1" => {
        day_1::step_1();
        day_1::step_2();
      },
      "2" => {
        day_2::step_1();
        day_2::step_2();
      },
      "3" => {
        day_3::step_1();
        day_3::step_2();
      }
      &_ => println!("Day {} solution not available", args[1].as_str()),
    }
  }
}

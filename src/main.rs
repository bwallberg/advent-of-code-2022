use std::env;

mod advent;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    day_4::main();
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
      },
      "4" => {
        day_4::main();
      }
      &_ => println!("Day {} solution not available", args[1].as_str()),
    }
  }
}

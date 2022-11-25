use std::env;

mod advent;
mod day_1;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    day_1::main();
  } else {
    match args[1].as_str() {
      "1" => day_1::main(),
      &_ => println!("Day {} solution not available", args[1].as_str()),
    }
  }
}

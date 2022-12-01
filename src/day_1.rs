use crate::advent;

fn get_elves(input: Vec<String>) -> Vec<i32> {
  let mut elves: Vec<i32> = input
    .split(|calories| calories == "")
    .map(|elf| elf.into_iter())
    .map(|elf| {
      elf
        .map(|calorie| calorie.parse::<i32>().unwrap())
        .reduce(|total, calorie| total + calorie)
        .unwrap()
    })
    .collect();

  elves.sort();
  elves.reverse();

  return elves;
}

pub fn step_1() {
  let input = advent::read_lines("1").unwrap();

  println!("Solution Step 1: {}", get_elves(input).first().unwrap());
}

pub fn step_2() {
  let input = advent::read_lines("1").unwrap();

  let elves = get_elves(input);
  println!("Solution Step 1: {}", elves[0] + elves[1] + elves[2]);
}

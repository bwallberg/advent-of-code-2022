use crate::advent;

pub fn main() {
  let input = advent::read_lines("4").unwrap();
  // TODO: figure out how to clone a Vec<String>...
  let input_step_2 = advent::read_lines("4").unwrap();

  println!("Solution Step 1: {}", step_1(input));
  println!("Solution Step 2: {}", step_2(input_step_2));
}

fn fully_contained(a: &(i32, i32), b: &(i32, i32)) -> bool {
  if a.0 >= b.0 && a.1 <= b.1 {
    return true;
  }
  false
}

fn overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
  if a.0 <= b.1 && a.1 >= b.0 {
    return true;
  }
  false
}

fn get_sections(input: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
  input
    .iter()
    .map(|l| l.split(',').collect::<Vec<&str>>())
    .map(|elves| {
      elves.into_iter().map(|elf| {
        let mut sections = elf.split('-');
        (
          sections.next().unwrap().parse().unwrap(),
          sections.next().unwrap().parse().unwrap(),
        )
      })
    })
    .map(|mut elves| (elves.next().unwrap(), elves.next().unwrap()))
    .collect()
}

fn step_1(input: Vec<String>) -> i32 {
  let mut total = 0;
  get_sections(input).iter().for_each(|(first, second)| {
    if fully_contained(first, second) || fully_contained(second, first) {
      total += 1;
    }
  });

  total
}

#[test]
fn test_step_1() {
  let input: Vec<String> = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    .lines()
    .map(String::from)
    .collect();

  assert_eq!(step_1(input), 2);
}

fn step_2(input: Vec<String>) -> i32 {
  let mut total = 0;
  get_sections(input).iter().for_each(|(first, second)| {
    if overlap(first, second) {
      total += 1;
    }
  });

  total
}

#[test]
fn test_step_2() {
  let input: Vec<String> = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    .lines()
    .map(String::from)
    .collect();

  assert_eq!(step_2(input), 4);
}

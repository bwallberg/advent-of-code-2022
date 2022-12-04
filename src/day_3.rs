use crate::advent::read_lines;

fn get_duplicate(rucksack: &String) -> Option<char> {
  let (first, second) = rucksack.split_at(rucksack.len() / 2);
  first.chars().find(|item| second.contains(*item))
}

fn get_groups(input: Vec<String>) -> Vec<char> {
  let iter = input.chunks(3);

  let mut groups: Vec<char> = Vec::new();
  for chunk in iter {
    let group_item = chunk[0]
      .chars()
      .find(|item| chunk[1].contains(*item) && chunk[2].contains(*item))
      .unwrap();
    groups.push(group_item)
  }

  groups
}

fn get_item_priority(item: char) -> i32 {
  let mut priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
  (priority.position(|letter| letter == item).unwrap() + 1)
    .try_into()
    .unwrap()
}

pub fn step_1() {
  let sum: i32 = read_lines("3")
    .unwrap()
    .into_iter()
    .map(String::from)
    .map(|rucksack| get_duplicate(&rucksack).unwrap())
    .map(get_item_priority)
    .sum();

  println!("Solution Step 1: {}", sum);
}

#[test]
pub fn step_1_test() {
  let sum: i32 = "vJrwpWtwJgWrhcsFMMfFFhFp
  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  PmmdzqPrVvPwwTWBwg
  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
  ttgJtRGJQctTZtZT
  CrZsJsPPZsGzwwsLwLmpwMDw"
    .lines()
    .map(String::from)
    .map(|rucksack| get_duplicate(&rucksack).unwrap())
    .map(get_item_priority)
    .sum();

  assert_eq!(sum, 157)
}

pub fn step_2() {
  let input = read_lines("3").unwrap();
  let sum: i32 = get_groups(input).into_iter().map(get_item_priority).sum();

  println!("Solution Step 2: {}", sum);
}

#[test]
pub fn step_2_test() {
  let input: Vec<String> = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    .lines()
    .map(String::from)
    .collect();

  let sum: i32 = get_groups(input).into_iter().map(get_item_priority).sum();

  assert_eq!(sum, 70)
}

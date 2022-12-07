use crate::advent;
use itertools::Itertools;

pub fn main() {
  let input = advent::read("6").unwrap();

  println!("Solution Step 1: {}", step_1(&input));
  println!("Solution Step 2: {}", step_2(&input));
}

fn step_1(input: &str) -> i32 {
  let mut marker: String = String::new();
  let (i, _) = input.chars().enumerate().find(|(_, c)| {
      marker.push(*c);
      if marker.chars().into_iter().unique().count() == 4 {
        return true
      } else if marker.len() == 4 {
        marker.remove(0);
      }
      false
  }).unwrap();
  (i + 1) as i32
}

#[test]
fn step_1_test() { 
  let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

  assert_eq!(step_1(&input), 10);
}

fn step_2(input: &str) -> i32 {
  let mut marker: String = String::new();
  let (i, _) = input.chars().enumerate().find(|(_, c)| {
      marker.push(*c);
      if marker.chars().into_iter().unique().count() == 14 {
        return true
      } else if marker.len() == 14 {
        marker.remove(0);
      }
      false
  }).unwrap();
  (i + 1) as i32
}

#[test]
fn step_2_test() { 
  let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

  assert_eq!(step_2(&input), 29);
}

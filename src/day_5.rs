// this is horrible

use std::collections::VecDeque;

use crate::advent;

type Cargo = Vec<VecDeque<String>>;
type Moves = Vec<(i32, i32, i32)>;

fn get_cargo_and_moves(input: &[String]) -> (Cargo, Moves) {
  let mut input_split = input.split(|l| l.is_empty());

  let (cargo, moves) = (input_split.next().unwrap(), input_split.next().unwrap());
  let (stacks, cargo) = cargo.split_last().unwrap();

  let stacks = stacks.replace(' ', "");
  let mut stacks_cargo: Vec<VecDeque<String>> = Vec::new();

  for _ in 0..stacks.len() {
    stacks_cargo.push(VecDeque::new());
  }

  cargo.iter().for_each(|c| {
    let mut stack_index = 0;
    c.chars().enumerate().for_each(|(i, c)| {
      if i % 4 == 1 {
        if !c.is_whitespace() {
          stacks_cargo[stack_index].push_front(String::from(c));
        }
        stack_index += 1;
      }
    });
  });

  let moves: Vec<String> = moves
    .iter()
    .map(|l| {
      l.replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
    })
    .collect();
  let moves: Vec<(i32, i32, i32)> = moves
    .iter()
    .map(|l| {
      let split: Vec<&str> = l.split(' ').into_iter().collect();
      (
        split[0].parse().unwrap(),
        split[1].parse().unwrap(),
        split[2].parse().unwrap(),
      )
    })
    .collect();

  (stacks_cargo, moves)
}

pub fn main() {
  let input = advent::read_lines("5").unwrap();

  println!("Solution Step 1: {}", step_1(&input));
  println!("Solution Step 2: {}", step_2(&input));
  // println!("Solution Step 2: {}", step_2(input));
}

fn step_1(input: &[String]) -> String {
  let (mut cargo, moves) = get_cargo_and_moves(input);
  moves.into_iter().for_each(|(m, f, t)| {
    for _ in 0..m {
      if let Some(val) = cargo[(f - 1) as usize].pop_back() {
        cargo[(t - 1) as usize].push_back(val);
      }
    }
  });


  let mut top_row: String = String::from("");
  cargo.iter().for_each(|stack| {
    top_row.push_str(stack.iter().last().unwrap());
  });
  
  top_row
}

#[test]
fn step_1_test() {
  let input = advent::read_lines("5_test").unwrap();

  assert_eq!(step_1(&input), "CMZ");
}

fn step_2(input: &[String]) -> String {
  let (mut cargo, moves) = get_cargo_and_moves(input);
  moves.into_iter().for_each(|(m, f, t)| {
    let mut moved_cargo: Vec<String> = Vec::new();
    for _ in 0..m {
      if let Some(val) = cargo[(f - 1) as usize].pop_back() {
        moved_cargo.push(val)
      }
    }
    moved_cargo.reverse();
    moved_cargo.into_iter().for_each(|c| {
      cargo[(t - 1) as usize].push_back(c);
    })
  });


  let mut top_row: String = String::from("");
  cargo.iter().for_each(|stack| {
    top_row.push_str(stack.iter().last().unwrap());
  });
  
  top_row
}

#[test]
fn step_2_test() {
  let input = advent::read_lines("5_test").unwrap(); 

  assert_eq!(step_2(&input), "MCD");
}



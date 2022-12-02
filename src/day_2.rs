use crate::advent;

#[derive(Debug, PartialEq, Clone, Copy)]
enum HandShape {
  Rock,
  Paper,
  Scissor,
}

#[derive(Debug, PartialEq)]
enum MatchResult {
  LOSS,
  DRAW,
  WIN,
}

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSOR: i32 = 3;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn get_hand_shape(input: &str) -> Option<HandShape> {
  match input {
    "A" => Some(HandShape::Rock),
    "B" => Some(HandShape::Paper),
    "C" => Some(HandShape::Scissor),
    "X" => Some(HandShape::Rock),
    "Y" => Some(HandShape::Paper),
    "Z" => Some(HandShape::Scissor),
    _ => None,
  }
}

fn get_match_result(input: &str) -> Option<MatchResult> {
  match input {
    "X" => Some(MatchResult::LOSS),
    "Y" => Some(MatchResult::DRAW),
    "Z" => Some(MatchResult::WIN),
    _ => None,
  }
}

fn get_hand_shape_score(input: &HandShape) -> i32 {
  match input {
    HandShape::Rock => ROCK,
    HandShape::Paper => PAPER,
    HandShape::Scissor => SCISSOR,
  }
}

fn get_hand_shape_by_result(opponent: &HandShape, result: MatchResult) -> HandShape {
  if result == MatchResult::DRAW {
    return opponent.clone();
  }
  if result == MatchResult::WIN {
    return match opponent {
      HandShape::Rock => HandShape::Paper,
      HandShape::Paper => HandShape::Scissor,
      HandShape::Scissor => HandShape::Rock,
    };
  }
  return match opponent {
    HandShape::Rock => HandShape::Scissor,
    HandShape::Paper => HandShape::Rock,
    HandShape::Scissor => HandShape::Paper,
  };
}

fn get_outcome_score(a: &HandShape, b: &HandShape) -> i32 {
  if *a == *b {
    return DRAW;
  }
  if (*a == HandShape::Rock && *b == HandShape::Scissor)
    || (*a == HandShape::Paper && *b == HandShape::Rock)
    || (*a == HandShape::Scissor && *b == HandShape::Paper)
  {
    return WIN;
  }
  return LOSS;
}

fn get_total_score_step_1(input: Vec<(String, String)>) -> i32 {
  let mut score = 0;
  for (col_1, col_2) in input {
    let opponent = get_hand_shape(col_1.as_str()).unwrap();
    let player = get_hand_shape(col_2.as_str()).unwrap();

    score += get_hand_shape_score(&player) + get_outcome_score(&player, &opponent)
  }

  return score;
}

fn get_total_score_step_2(input: Vec<(String, String)>) -> i32 {
  let mut score = 0;
  for (col_1, col_2) in input {
    let opponent = get_hand_shape(col_1.as_str()).unwrap();
    let result = get_match_result(col_2.as_str()).unwrap();
    let player = get_hand_shape_by_result(&opponent, result);

    score += get_hand_shape_score(&player) + get_outcome_score(&player, &opponent)
  }

  return score;
}

fn get_strategy_guide(input: Vec<String>) -> Vec<(String, String)> {
  let mut strategy_guide = Vec::new();
  input.into_iter().for_each(|line| {
    let mut values = line.split(" ");
    let (col_1, col_2) = (values.next().unwrap(), values.next().unwrap());
    strategy_guide.push((col_1.to_string(), col_2.to_string()));
  });

  return strategy_guide;
}

pub fn step_1() {
  let input = advent::read_lines("2").unwrap();
  let strategy_guide = get_strategy_guide(input);

  println!(
    "Solution Step 1: {:?}",
    get_total_score_step_1(strategy_guide)
  );
}

#[test]
fn test_step_1() {
  let input = Vec::from([
    (String::from("A"), String::from("Y")),
    (String::from("B"), String::from("X")),
    (String::from("C"), String::from("Z")),
  ]);

  assert_eq!(get_total_score_step_2(input), 15);
}

pub fn step_2() {
  let input = advent::read_lines("2").unwrap();
  let strategy_guide = get_strategy_guide(input);

  println!(
    "Solution Step 2: {:?}",
    get_total_score_step_2(strategy_guide)
  );
}

#[test]
fn test_step_2() {
  let input = Vec::from([
    (String::from("A"), String::from("Y")),
    (String::from("B"), String::from("X")),
    (String::from("C"), String::from("Z")),
  ]);

  assert_eq!(get_total_score_step_2(input), 12);
}

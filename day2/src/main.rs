use std::{io::{stdin, BufReader, BufRead}};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Move {
  Rock,
  Paper,
  Sizzors,
}

#[derive(Debug, PartialEq)]
struct ParseMoveError;

impl FromStr for Move {
  type Err = ParseMoveError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok( match s {
      "A" => Move::Rock,
      "B" => Move::Paper,
      "C" => Move::Sizzors,
      "X" => Move::Rock,
      "Y" => Move::Paper,
      "Z" => Move::Sizzors,
      _ => return Err(ParseMoveError),
    })
  }
}

#[derive(Debug, PartialEq)]
enum Winner {
  Lose,
  Draw,
  Win,
}

#[derive(Debug, PartialEq)]
struct ParseWinnerError;

impl FromStr for Winner {
  type Err = ParseWinnerError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok( match s {
      "X" => Winner::Lose,
      "Y" => Winner::Draw,
      "Z" => Winner::Win,
      _ => return Err(ParseWinnerError),
    })
  }
}

fn main() {
  let br = BufReader::new(stdin().lock());
  let strategy= &mut br.lines().map( |v| v.unwrap()).collect::<Vec<String>>();
  let mut score = 0;
  for moves in strategy{
    let moves = moves.split_once(' ').unwrap();
    let (opp_move, winner) = ( moves.0.parse::<Move>().unwrap(), moves.1.parse::<Winner>().unwrap());

    use Move::*;
    use Winner::*;

    let my_move = match (&opp_move, &winner) {
      (Paper, Lose) => Rock,
      (Paper, Draw) => Paper,
      (Paper, Win) => Sizzors,
      (Rock, Lose) => Sizzors,
      (Rock, Draw) => Rock,
      (Rock, Win) => Paper,
      (Sizzors, Lose) => Paper,
      (Sizzors, Draw) => Sizzors,
      (Sizzors, Win) => Rock,
    };

    score += match &my_move {
      Rock => 1,
      Paper => 2,
      Sizzors => 3,
    };

    score += match (&opp_move, &my_move) {
      (Paper, Paper) => 3,
      (Paper, Rock) => 0,
      (Paper, Sizzors) => 6,
      (Rock, Paper) => 6,
      (Rock, Rock) => 3,
      (Rock, Sizzors) => 0,
      (Sizzors, Paper) => 0,
      (Sizzors, Rock) => 6,
      (Sizzors, Sizzors) => 3,
    };
  }

  println!("{:?}", score);
}

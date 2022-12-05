use std::{io::{stdin, BufReader, BufRead}};

const BIG_A : u32 = 'A' as u32;
const LITTLE_A : u32 = 'a' as u32;

fn main() {
  let br = BufReader::new(stdin().lock());
  let lines = &mut br.lines().map( |v| v.unwrap()).collect::<Vec<String>>();
  let mut score : u64 = 0;

  let mut it = lines.iter();
  while let Some(one) = it.next() {
    let two = it.next().unwrap();
    let three = it.next().unwrap();


    for c in one.chars() {
      if two.contains(c) && three.contains(c) {
        let priority = match c {
          'A'..='Z' => (c as u32) - BIG_A + 27,
          'a'..='z' => (c as u32) - LITTLE_A + 1,
          x         => panic!("shouldn't get here '{}'", x),
        };
        score += priority as u64;
        break;
      }
    }
  }

  println!("{:?}", score);
}

use std::{io::{stdin, BufReader, BufRead}};
use std::collections::BinaryHeap;

fn main() {
  let br = BufReader::new(stdin().lock());
  let calories = &mut br.lines().map( |v| v.unwrap().parse::<i32>().ok()).collect::<Vec<Option<i32>>>();
  let mut grouped_by_and_summed : BinaryHeap::<i32> =
    calories.split(|v| v.is_none()).map( |elf| elf.iter().map( |v| v.unwrap()).sum()).collect();

  let mut value = 0;
  value += grouped_by_and_summed.pop().unwrap();
  value += grouped_by_and_summed.pop().unwrap();
  value += grouped_by_and_summed.pop().unwrap();

  println!("{:?}", value);
}

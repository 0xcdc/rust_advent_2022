use std::{io::{stdin, BufReader, BufRead}};
use std::collections::VecDeque;

const TARGET_UNIQUE_CHARS : usize = 14;

fn main() {
  let br = BufReader::new(stdin().lock());
  let lines_it = &mut br.lines().map( |v| v.unwrap());

  let mut lookahead : VecDeque<char> = VecDeque::new();
  let mut count : usize = 0;

  if let Some(line) = lines_it.next() {
    let line_it = line.chars();

    for c in line_it {
      //we want to push unique characters and pop non-unique characters
      //when we get n unique characters we are done
      println!("{:?} {:?}", lookahead, c);
      count += 1;
      //check if c is in the lookahead already
      if let Some(pos) = lookahead.iter().position(|v| *v == c) {
        //remove pos + 1 elements (position is 0 based)
        lookahead.drain(0..pos+1);
      }
      lookahead.push_back(c);
      if lookahead.len() == TARGET_UNIQUE_CHARS {
        println!("{:?}", count);
        return;
      }
    }
  }

  panic!("did not find marker");
}

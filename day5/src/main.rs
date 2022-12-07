use std::{io::{stdin, BufReader, BufRead}};

fn main() {
  let br = BufReader::new(stdin().lock());
  let lines_it = &mut br.lines().map( |v| v.unwrap());

  let mut stacks : Vec<Vec<char>> = Vec::<Vec<char>>::new();
  for line in lines_it.by_ref() {
    //process to the blank line that signals the move instructions
    if line.is_empty() { break; }

    println!("{}", line);
    let mut line_it = line.chars();
    let mut stack_no = 0usize;
    while let Some(c) = line_it.next() {
      let d = line_it.next().unwrap();

      if c != '[' && d != ' ' {
        //we're out of the stack diagram, ignore the number line
        break;
      }

      while stacks.len() <= stack_no  {
        stacks.push(Vec::<char>::new());
      }

      if d != ' ' {
        stacks[stack_no].insert(0, d);
      }

      line_it.next().unwrap();

      if line_it.next().is_none() { break; }

      stack_no += 1;
    }
  }

  for line in lines_it.by_ref() {
    let mut word_it = line.split(' ');
    //skip the move
    word_it.next();

    //number of boxes to move
    let count = word_it.next().unwrap().parse::<usize>().unwrap();

    //skip from
    word_it.next();

    let source = word_it.next().unwrap().parse::<usize>().unwrap() - 1;

    //skip to
    word_it.next();

    let dest= word_it.next().unwrap().parse::<usize>().unwrap() - 1;

    let first_index = stacks[source].len() - count;
    let last_index = stacks[source].len();

    for i in first_index..last_index {
      let item = stacks[source][i];
      stacks[dest].push(item);
    }

    stacks[source].resize(first_index, '\0');
  }

  let mut result : String = String::new();
  for stack in &stacks {
    result.push( stack[stack.len()-1]);
  }

  println!("{:?}", result);
}

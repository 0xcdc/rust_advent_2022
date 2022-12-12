use std::{io::{stdin, BufReader, BufRead}};
use std::iter::Peekable;
use std::collections::HashMap;

fn process_dir_listing<I>(lines_it : &mut Peekable<I>) -> usize
  where I : Iterator<Item = String> {

    let mut total_size = 0;

    while let Some(line) = lines_it.next_if(|v| !v.starts_with('$')) {
      if line.starts_with("dir ") {
        //ignore directors b/c we must cd into them to find their size
      } else {
        let mut parts = line.split(' ');
        let size = parts.next().unwrap();
        let size = size.parse::<usize>().unwrap();
        let _filename = parts.next().unwrap();
        assert!(parts.next().is_none());
        total_size += size;
      }
    }

    total_size
}

fn main() {
  let mut sizes : HashMap<Vec<String>, usize> = HashMap::new();

  let br = BufReader::new(stdin().lock());
  let mut lines_it = br.lines().map( |v| v.unwrap()).peekable();

  assert!(lines_it.next() == Some("$ cd /".to_string()));
  let mut path = Vec::<String>::new();
  sizes.insert(path.clone(), 0);

  //process all the input
  while let Some(line) = lines_it.next() {
    if line == "$ cd .." {
      let dir_size = sizes[&path];
      path.pop();
      *sizes.get_mut(&path).unwrap() += dir_size;
    } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
      path.push(dir_name.to_string());
      sizes.insert(path.clone(), 0);
    } else {
      assert!(line == "$ ls", "{}", line);
      let size = process_dir_listing(&mut lines_it);
      *sizes.get_mut(&path).unwrap() += size;
    }
  }

  //we may have ended without cd ..'ing to root
  while !path.is_empty() {
    let dir_size = sizes[&path];
    path.pop();
    *sizes.get_mut(&path).unwrap() += dir_size;
  }

  assert!(path.is_empty());
  let total_used = sizes[&path];
  let free_space = 70_000_000 - total_used;
  let space_needed = 30_000_000 - free_space;
  let mut answer = usize::MAX;
  for value in sizes.values() {
    if *value >= space_needed && *value < answer {
      answer = *value;
    }
  }

  println!("{}", answer);
}

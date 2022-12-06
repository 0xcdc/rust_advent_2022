use std::{io::{stdin, BufReader, BufRead}};

fn contains(x: &[u32], y: &[u32]) -> bool {
  (x[0] <= y[0]) && (x[1] >= y[1])
}

fn overlaps(x: &[u32], y: &[u32]) -> bool {
  let (s1, e1, s2, e2) = (x[0], x[1], y[0], y[1]);

  // is 1 the first range?
  if s1 < s2 {
    // 1 is the first range
    // does the end of 1 reach the start of 2
    if e1 >= s2 { return true; }
  } else {
    // 2 is the first range
    // does the end of 2 reach past the start of 1
    if e2 >= s1 { return true; }
  }
  false
}


fn main() {
  let br = BufReader::new(stdin().lock());
  let lines = &mut br.lines().map( |v| v.unwrap()).collect::<Vec<String>>();
  let mut score1: u64 = 0;
  let mut score2: u64 = 0;

  let mut it = lines.iter();
  while let Some(line) = it.next() {
    let elfs = line.split(',').collect::<Vec<&str>>();

    let elf1 = elfs[0].split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let elf2 = elfs[1].split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    if contains(&elf1, &elf2) || contains(&elf2, &elf1) {
      score1 += 1;
    }

    if overlaps(&elf1, &elf2) {
      score2 += 1;
    }
  }

  println!("{:?} {:?}", score1, score2);
}

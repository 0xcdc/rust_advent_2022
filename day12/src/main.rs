use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{stdin, BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Board {
    data: Vec<char>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Board {
    fn new(data: Vec<char>, width: usize, height: usize) -> Self {
        let spos = data.iter().position(|&v| v == 'S').unwrap();
        let start = Point {
            x: spos % width,
            y: spos / width,
        };
        let epos = data.iter().position(|&v| v == 'E').unwrap();
        let end = Point {
            x: epos % width,
            y: epos / width,
        };
        Board {
            data,
            width,
            height,
            start,
            end,
        }
    }

    fn get(self: &Board, p: &Point) -> char {
        self.data[p.x + p.y * self.width]
    }

    fn get_height(self: &Board, p: &Point) -> i32 {
        let c = self.get(p);
        let result = match c {
            'S' => 0,
            'E' => (b'z' - b'a') as i32,
            'a'..='z' => (c as u8 - b'a') as i32,
            _ => panic!("unexpected char"),
        };
        result
    }

    fn adjacent_points(self: &Board, p: &Point) -> Vec<Point> {
        let mut result = vec![];
        if p.x >= 1 {
            result.push(Point { x: p.x - 1, y: p.y })
        };
        if p.x < self.width - 1 {
            result.push(Point { x: p.x + 1, y: p.y })
        };
        if p.y >= 1 {
            result.push(Point { x: p.x, y: p.y - 1 })
        };
        if p.y < self.height - 1 {
            result.push(Point { x: p.x, y: p.y + 1 })
        };

        result.retain(|v| self.get_height(v) - self.get_height(p) <= 1);
        result
    }

    fn dump_visited(self: &Board, already_visited: &HashSet<Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point { x, y };
                print!(
                    "{}",
                    if already_visited.get(&p).is_some() {
                        self.get(&p)
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
        println!("--------------------------------------------------");
    }

    fn dump_path(self: &Board, path: &[Point]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point { x, y };
                print!("{}", if path.contains(&p) { self.get(&p) } else { '.' });
            }
            println!();
        }
        println!("--------------------------------------------------");
    }
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let mut data = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in lines_it {
        let mut w = 0;
        for c in line.chars() {
            data.push(c);
            w += 1;
        }
        if width == 0 {
            width = w;
        }
        assert_eq!(w, width);
        height += 1;
    }

    let mut answer = usize::MAX;
    let map = Board::new(data, width, height);

    for y in 0..map.height {
        for x in 0..map.width {
            let start = Point { x, y };
            if map.get_height(&start) != 0 {
                continue;
            }
            let mut already_visited = HashSet::<Point>::new();
            let mut working_set = VecDeque::<Vec<Point>>::new();
            working_set.push_back(vec![start]);
            already_visited.insert(start);

            while let Some(path) = working_set.pop_front() {
                let last_point = *path.last().unwrap();
                if last_point == map.end {
                    if answer > path.len() - 1 {
                        map.dump_path(&path);
                        answer = path.len() - 1;
                    }
                    break;
                }

                for p in map.adjacent_points(&last_point) {
                    if already_visited.contains(&p) {
                        continue;
                    }

                    let mut new_path = path.clone();
                    new_path.push(p);
                    already_visited.insert(p);
                    working_set.push_back(new_path);
                }
            }
        }
    }

    println!("{}", answer);
}

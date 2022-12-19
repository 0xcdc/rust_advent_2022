use crossterm::{cursor, style::Print, QueueableCommand};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::{thread, time};

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

    fn update_display(self: &Board, s: &str, last_board: &mut String) {
        let mut stdout = stdout();
        let mut old_chars = last_board.chars();
        let mut new_chars = s.chars();
        for y in 0..self.height {
            for x in 0..self.width {
                let old = old_chars.next().unwrap();
                let new = new_chars.next().unwrap();

                if old != new {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))
                        .unwrap()
                        .queue(Print(new))
                        .unwrap();
                }
            }
        }
        stdout
            .queue(cursor::MoveTo(0, self.height as u16 + 1))
            .unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(1));
        last_board.truncate(0);
        last_board.push_str(s);
    }

    fn dump_visited(self: &Board, already_visited: &HashSet<Point>, last_board: &mut String) {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point { x, y };
                s.push(if already_visited.contains(&p) {
                    self.get(&p)
                } else {
                    '.'
                });
            }
        }

        self.update_display(&s, last_board);
    }

    fn dump_path(self: &Board, path: &[Point], last_board: &mut String) {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point { x, y };
                s.push(if path.contains(&p) { self.get(&p) } else { '.' });
            }
        }

        self.update_display(&s, last_board);
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
    let mut path_length = 0;
    let mut last_board = String::from_utf8(vec![b' '; height * width]).unwrap();

    /*    for y in 0..map.height {
    for x in 0..map.width {*/
    for y in map.start.y..=map.start.y {
        for x in map.start.x..=map.start.x {
            let start = Point { x, y };
            if map.get_height(&start) != 0 {
                continue;
            }
            let mut already_visited = HashSet::<Point>::new();
            let mut working_set = VecDeque::<Vec<Point>>::new();
            working_set.push_back(vec![start]);
            already_visited.insert(start);

            while let Some(path) = working_set.pop_front() {
                map.dump_visited(&already_visited, &mut last_board);
                if path.len() > path_length {
                    path_length = path.len();
                    //map.dump_path(&path, &mut last_board);
                }
                let last_point = *path.last().unwrap();
                if last_point == map.end {
                    if answer > path.len() - 1 {
                        map.dump_path(&path, &mut last_board);
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

    println!();
    println!("{}", answer);
}

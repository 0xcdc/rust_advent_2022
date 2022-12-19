use std::io::{stdin, BufRead, BufReader};

const SAND: (i32, i32) = (500, 0);

#[derive(Debug)]
struct Board {
    top: i32,
    left: i32,
    height: i32,
    width: i32,
    data: Vec<u8>,
}

impl Board {
    fn new(segments: Vec<Vec<(i32, i32)>>) -> Self {
        let mut segments = segments;
        let mut min_x = SAND.0;
        let mut min_y = SAND.1;
        let mut max_x = SAND.0;
        let mut max_y = SAND.1;

        for wall in &segments {
            for seg in wall {
                let (x, y) = *seg;
                min_x = min_x.min(x);
                min_y = min_y.min(y);

                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }

        let top = min_y;
        let height = max_y - min_y + 1 + 2;

        min_x = min_x.min(SAND.0 - height);
        max_x = max_x.max(SAND.0 + height);

        let left = min_x;
        let width = max_x - min_x + 1;

        segments.push(vec![(left, height - 1), (left + width - 1, height - 1)]);

        println!("{:?}", (top, left, height, width));
        let mut result = Board {
            top,
            left,
            height,
            width,
            data: vec![b'.'; (height * width) as usize],
        };

        for wall in &segments {
            let mut wall_it = wall.iter();
            let mut old = wall_it.next().unwrap();
            for new in wall_it {
                for x in old.0.min(new.0)..=old.0.max(new.0) {
                    for y in old.1.min(new.1)..=old.1.max(new.1) {
                        result.put(x, y, b'#');
                    }
                }
                old = new;
            }
        }

        result
    }

    fn put(self: &mut Board, x: i32, y: i32, c: u8) {
        let x = x - self.left;
        let y = y - self.top;
        self.data[(x + y * self.width) as usize] = c;
    }

    fn get(self: &Board, x: i32, y: i32) -> u8 {
        let x = x - self.left;
        let y = y - self.top;
        self.data[(x + y * self.width) as usize]
    }

    fn add_sand(self: &mut Board) -> bool {
        let mut pos = SAND;
        if self.get(pos.0, pos.1) == b'o' {
            return false;
        }
        loop {
            if pos.1 == self.height - 1 {
                return false;
            }

            if self.get(pos.0, pos.1 + 1) == b'.' {
                pos.1 += 1;
            } else if self.get(pos.0 - 1, pos.1 + 1) == b'.' {
                pos.0 -= 1;
                pos.1 += 1;
            } else if self.get(pos.0 + 1, pos.1 + 1) == b'.' {
                pos.0 += 1;
                pos.1 += 1;
            } else {
                self.put(pos.0, pos.1, b'o');
                return true;
            }
        }
    }

    fn dump(self: &Board) {
        let mut s = String::new();
        for y in self.top..self.top + self.height {
            for x in self.left..self.left + self.width {
                s.push(self.get(x, y) as char);
            }
            s.push('\n');
        }

        println!("{}", s);
    }
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let mut lines_it = br.lines().map(|v| v.unwrap());

    let mut segments: Vec<Vec<(i32, i32)>> = Vec::new();
    while let Some(line) = lines_it.next() {
        let segment: Vec<Vec<i32>> = line
            .split(" -> ")
            .map(|p| p.split(',').map(|v| v.parse::<i32>().unwrap()).collect())
            .collect();
        let segment = segment
            .iter()
            .map(|v| {
                assert_eq!(v.len(), 2);
                (v[0], v[1])
            })
            .collect();
        segments.push(segment);
    }

    let mut board = Board::new(segments);
    let mut answer = 0;
    while board.add_sand() {
        //board.dump();
        answer += 1;
    }

    println!("answer 1 -> {}", answer);
}

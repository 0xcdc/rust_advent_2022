use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

struct State {
    rope: Vec<Coord>,
    visited: HashSet<Coord>,
}

#[derive(Debug, PartialEq)]
enum Direction {
    NoMove,
    OneStep,
    Diagonal,
}

fn tail_direction(a: &Coord, b: &Coord) -> Direction {
    let delta = ((a.x - b.x).abs(), (a.y - b.y).abs());
    match delta {
        (0, 0) => Direction::NoMove,
        (0, 1) => Direction::NoMove,
        (1, 0) => Direction::NoMove,
        (1, 1) => Direction::NoMove,
        (0, _) => Direction::OneStep,
        (_, 0) => Direction::OneStep,
        _ => Direction::Diagonal,
    }
}

fn move_tail(state: &mut State) {
    for i in 0..state.rope.len() - 1 {
        let cur = state.rope[i];
        let next = &mut state.rope[i + 1];
        let d = tail_direction(&cur, next);

        match d {
            Direction::NoMove => { /* no-op */ }
            Direction::OneStep => {
                if cur.x > next.x {
                    next.x += 1;
                } else if cur.x < next.x {
                    next.x -= 1;
                } else if cur.y > next.y {
                    next.y += 1;
                } else {
                    assert!(cur.y < next.y);
                    next.y -= 1;
                }
            }
            Direction::Diagonal => {
                if cur.x > next.x {
                    next.x += 1;
                } else {
                    assert!(cur.x < next.x);
                    next.x -= 1;
                }

                if cur.y > next.y {
                    next.y += 1;
                } else {
                    assert!(cur.y < next.y);
                    next.y -= 1;
                }
            }
        }
    }
    state.visited.insert(*state.rope.last().unwrap());
}

fn move_up(state: &mut State) {
    state.rope[0].y += 1;
    move_tail(state);
}

fn move_down(state: &mut State) {
    state.rope[0].y -= 1;
    move_tail(state);
}

fn move_left(state: &mut State) {
    state.rope[0].x -= 1;
    move_tail(state);
}

fn move_right(state: &mut State) {
    state.rope[0].x += 1;
    move_tail(state);
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let mut state = State {
        rope: vec![Coord { x: 0, y: 0 }; 10],
        visited: HashSet::<Coord>::new(),
    };

    for line in lines_it {
        let mut parts = line.split(' ');
        let dir = parts.next().unwrap();
        let mut count = parts.next().unwrap().parse::<i32>().unwrap();
        assert_eq!(parts.next(), None);

        while count > 0 {
            match dir {
                "R" => move_right(&mut state),
                "L" => move_left(&mut state),
                "U" => move_up(&mut state),
                "D" => move_down(&mut state),
                _ => panic!("unexpected direction"),
            }
            count -= 1;
        }
    }

    let answer = state.visited.len();

    println!("{}", answer);
}

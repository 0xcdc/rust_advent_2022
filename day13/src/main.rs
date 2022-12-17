use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::io::{stdin, BufRead, BufReader};
use std::iter::Peekable;

#[derive(Clone, Eq, PartialEq)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::List(l) => {
                let s = l.iter().map(|v| format!("{}", v)).collect::<Vec<String>>();
                write!(f, "[{}]", s.join(","))
            }
        }
    }
}

impl Value {
    fn parse_list<T>(chars: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut l = Vec::<Value>::new();

        let open_bracket = chars.next().unwrap();
        assert_eq!(open_bracket, '[');

        //guard against an empty list
        if *chars.peek().unwrap() == ']' {
            chars.next();
        } else {
            loop {
                let v = Self::parse(chars);
                l.push(v);

                let c = chars.next().unwrap();
                match c {
                    ']' => break,
                    ',' => { /* no-op */ }
                    _ => panic!("unexpected list delimiter"),
                }
            }
        }

        Value::List(l)
    }

    fn parse_integer<T>(chars: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut i_str = String::new();
        while let Some(c) = chars.peek() {
            match c {
                ',' | ']' => break,
                _ => i_str.push(*c),
            }
            chars.next();
        }

        let i: i32 = i_str.parse().unwrap();
        Value::Int(i)
    }

    fn parse<T>(chars: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let c = chars.peek().unwrap();
        match c {
            '[' => Self::parse_list(chars),
            _ => Self::parse_integer(chars),
        }
    }

    fn parse_str(s: &str) -> Self {
        Self::parse(&mut s.chars().peekable())
    }
}

impl Ord for Value {
    fn cmp(self: &Value, rhs: &Value) -> Ordering {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l.cmp(r),
            (Value::List(l), Value::List(r)) => l.iter().cmp(r.iter()),
            (Value::Int(l), r) => Value::List(vec![Value::Int(*l)]).cmp(r),
            (l, Value::Int(r)) => l.cmp(&Value::List(vec![Value::Int(*r)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(self: &Value, rhs: &Value) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let mut lines_it = br.lines().map(|v| v.unwrap());

    let mut answer = 0;
    let mut i = 0;
    let mut packets = vec![];
    while let Some(line) = lines_it.next() {
        i += 1;
        let left = Value::parse_str(&line);

        let line = lines_it.next().unwrap();
        let right = Value::parse_str(&line);

        if let Some(line) = lines_it.next() {
            assert_eq!(line, "");
        }

        if left.cmp(&right) == Ordering::Less {
            answer += i;
        }
        packets.push(left);
        packets.push(right);
    }

    println!("answer 1 -> {}", answer);

    let divider_packets = ["[[2]]", "[[6]]"].map(Value::parse_str);
    for d in &divider_packets {
        packets.push(d.clone());
    }
    packets.sort();

    let positions = divider_packets.map(|v| packets.iter().position(|v2| v2 == &v).unwrap() + 1);
    println!("answer 2 -> {}", positions[0] * positions[1]);
}

use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
enum Expr {
    Old,
    Int(i32),
    Op(char, Vec<Expr>),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    op: Expr,
    test: i32,
    on_true: i32,
    on_false: i32,
    inspection_count: i32,
}

impl Expr {
  fn eval(self : &Expr, old : i32) -> i32 {
    match self {
      Expr::Old => old,
      Expr::Int(i) => *i,
      Expr::Op(c, v) => {
        match c {
          '+' => v[0].eval(old) + v[1].eval(old),
          '-' => v[0].eval(old) - v[1].eval(old),
          '*' => v[0].eval(old) * v[1].eval(old),
          '/' => v[0].eval(old) / v[1].eval(old),
          _ => panic!("unexpected operator"),
        }
      },
    }
  }
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let mut lines_it = br.lines().map(|v| v.unwrap());

    let mut monkeys = vec![];

    while let Some(line) = lines_it.next() {
        if line == "" { continue; }
        assert!(line.starts_with("Monkey"));

        let line = lines_it.next().unwrap();
        let items: Vec<i32> = line
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();


        let line = lines_it.next().unwrap();
        let expr_text = line.strip_prefix("  Operation: new = ").unwrap().split(' ');
        let mut tokens : Vec<Expr> = expr_text.map(|term|  {
            match term {
              "old" => Expr::Old,
              "+" | "*" | "-" | "/" => Expr::Op(term.chars().next().unwrap(), vec![]),
              _ => Expr::Int(term.parse::<i32>().unwrap()),
            }
        }).collect();

        let op = match tokens.remove(1) {
          Expr::Op(op, mut c) => {
            c.push(tokens.remove(0));
            c.push(tokens.remove(0));
            assert_eq!(tokens.len(), 0);
            Expr::Op(op, c)
          },
          _ => panic!("middle token was not an operator"),
        };

        let line = lines_it.next().unwrap();
        let test = line.strip_prefix("  Test: divisible by ").unwrap().parse::<i32>().unwrap();

        let line = lines_it.next().unwrap();
        let on_true = line.strip_prefix("    If true: throw to monkey ").unwrap().parse::<i32>().unwrap();

        let line = lines_it.next().unwrap();
        let on_false = line.strip_prefix("    If false: throw to monkey ").unwrap().parse::<i32>().unwrap();

        monkeys.push(Monkey {
            items: items,
            op: op,
            test: test,
            on_true: on_true,
            on_false: on_false,
            inspection_count: 0,
        });
    }

    for _ in 0..20 {
      for m in &mut monkeys {
        for mut i in m.items.drain(..) {
          i = m.op.eval(i) / 3;
          if i % m.test == 0 {
          }
          else {
          }
        }
      }
    }

    let answer_1 = 0;
    println!("{}", answer_1);
}

use std::fmt::Debug;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
enum Expr {
    Old,
    Int(i64),
    Op(char, Vec<Expr>),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Expr,
    test: i64,
    on_true: usize,
    on_false: usize,
    inspection_count: i64,
}

impl Expr {
    fn eval(self: &Expr, old: i64) -> i64 {
        match self {
            Expr::Old => old,
            Expr::Int(i) => *i,
            Expr::Op(c, v) => match c {
                '+' => v[0].eval(old) + v[1].eval(old),
                '-' => v[0].eval(old) - v[1].eval(old),
                '*' => v[0].eval(old) * v[1].eval(old),
                '/' => v[0].eval(old) / v[1].eval(old),
                _ => panic!("unexpected operator"),
            },
        }
    }
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let mut lines_it = br.lines().map(|v| v.unwrap());

    let mut monkeys = vec![];

    while let Some(line) = lines_it.next() {
        if line.is_empty() {
            continue;
        }
        assert!(line.starts_with("Monkey"));

        let line = lines_it.next().unwrap();
        let items: Vec<i64> = line
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        let line = lines_it.next().unwrap();
        let expr_text = line.strip_prefix("  Operation: new = ").unwrap().split(' ');
        let mut tokens: Vec<Expr> = expr_text
            .map(|term| match term {
                "old" => Expr::Old,
                "+" | "*" | "-" | "/" => Expr::Op(term.chars().next().unwrap(), vec![]),
                _ => Expr::Int(term.parse::<i64>().unwrap()),
            })
            .collect();

        let op = match tokens.remove(1) {
            Expr::Op(op, mut c) => {
                c.push(tokens.remove(0));
                c.push(tokens.remove(0));
                assert_eq!(tokens.len(), 0);
                Expr::Op(op, c)
            }
            _ => panic!("middle token was not an operator"),
        };

        let line = lines_it.next().unwrap();
        let test = line
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let line = lines_it.next().unwrap();
        let on_true = line
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let line = lines_it.next().unwrap();
        let on_false = line
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let inspection_count = 0;

        monkeys.push(Monkey {
            items,
            op,
            test,
            on_true,
            on_false,
            inspection_count,
        });
    }

    let common_divisor: i64 = monkeys.iter().map(|m| m.test).product();

    for _turn_count in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut j) = monkeys[i].items.pop() {
                monkeys[i].inspection_count += 1;
                j = monkeys[i].op.eval(j) % common_divisor;
                let new_index = if j % monkeys[i].test == 0 {
                    monkeys[i].on_true
                } else {
                    monkeys[i].on_false
                };
                monkeys[new_index].items.push(j);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    let mut answer_1 = monkeys.pop().unwrap().inspection_count;
    answer_1 *= monkeys.pop().unwrap().inspection_count;
    println!("{}", answer_1);
}

use std::io::{stdin, BufRead, BufReader};

fn incr_clock(values: &mut Vec<i32>, clock: &mut usize, x: i32) {
    *clock += 1;
    values.push(x);
}

fn is_lit(pixel: i32, x: i32) -> bool {
    (pixel - x).abs() <= 1
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let mut x = 1;
    let mut clock: usize = 0;

    let mut values: Vec<i32> = vec![1];

    for line in lines_it {
        if line == "noop" {
            incr_clock(&mut values, &mut clock, x);
        } else {
            let val: i32 = line.strip_prefix("addx ").unwrap().parse().unwrap();
            incr_clock(&mut values, &mut clock, x);
            incr_clock(&mut values, &mut clock, x);
            x += val;
        }
    }

    //make sure we have at least 240 clocks
    while clock < 241 {
        incr_clock(&mut values, &mut clock, x);
    }

    let answer_1: i32 = (20..241).step_by(40).map(|i| i * values[i as usize]).sum();
    println!("{}", answer_1);

    let mut display = String::new();

    clock = 1;
    while clock < 241 {
        for pixel in 0..40 {
            x = values[clock];
            if is_lit(pixel, x) {
                display.push('#');
            } else {
                display.push('.');
            }

            clock += 1;
        }
        display.push('\n');
    }
    println!("{}", display);
}

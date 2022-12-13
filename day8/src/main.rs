use std::io::{stdin, BufRead, BufReader};
use std::iter;

fn left_indexes(i: usize, size: usize) -> Vec<usize> {
    let i_column = i % size;
    let row_start = i - i_column;

    //we want the indexes to originate at i
    let mut result: Vec<usize> = (row_start..i).collect();
    result.reverse();
    result
}

fn right_indexes(i: usize, size: usize) -> Vec<usize> {
    let i_column = i % size;
    let row_start = i - i_column;
    let row_end = row_start + size;
    (i + 1..row_end).collect()
}

fn above_indexes(mut i: usize, size: usize) -> Vec<usize> {
    iter::from_fn(move || {
        if i >= size {
            i -= size;
            Some(i)
        } else {
            None
        }
    })
    .collect()
}

fn below_indexes(mut i: usize, size: usize) -> Vec<usize> {
    iter::from_fn(move || {
        i += size;
        if i < size * size {
            Some(i)
        } else {
            None
        }
    })
    .collect()
}

fn is_visible(trees: &[u32], i: usize) -> bool {
    let size = (trees.len() as f64).sqrt() as usize;
    assert_eq!(size * size, trees.len());

    let indexes_to_check = [
        left_indexes(i, size),
        right_indexes(i, size),
        above_indexes(i, size),
        below_indexes(i, size),
    ];

    let tree_height = trees[i];
    for indexes in &indexes_to_check {
        //are all the trees in a diminsion shorter than the test tree?
        if indexes.iter().all(|v| trees[*v] < tree_height) {
            //short circuit
            return true;
        }
    }

    //if it's not visible from the 4 directions then it's not visible
    false
}

fn scenic_score(trees: &[u32], i: usize) -> i32 {
    let size = (trees.len() as f64).sqrt() as usize;
    assert_eq!(size * size, trees.len());

    let indexes_to_check = [
        left_indexes(i, size),
        right_indexes(i, size),
        above_indexes(i, size),
        below_indexes(i, size),
    ];

    let tree_height = trees[i];
    indexes_to_check
        .map(|indexes| {
            let mut c = 0;
            for h in indexes {
                c += 1;
                if trees[h] >= tree_height {
                    break;
                }
            }
            c
        })
        .iter()
        .product()
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let mut trees: Vec<u32> = vec![];
    for line in lines_it {
        for c in line.chars() {
            trees.push(c.to_digit(10).unwrap());
        }
    }

    let mut answer = 0;
    for i in 0..trees.len() {
        answer += is_visible(&trees, i) as i32;
    }

    println!("visible trees {}", answer);

    answer = 0;
    for i in 0..trees.len() {
        answer = answer.max(scenic_score(&trees, i));
    }

    println!("max scenic score {}", answer);
}

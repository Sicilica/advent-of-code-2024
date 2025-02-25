use std::{collections::HashMap, fs};

// Memo: run with `cargo run -p day01`
// Memo: made with `cargo new day01`
fn main() {
    let contents = fs::read_to_string("./day01/input.txt").expect("read file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for l in contents.lines() {
        if l.is_empty() {
            continue;
        }
        let mut iter = l.split_whitespace();
        let first = iter.next().expect("line split");
        let second = iter.next().expect("line split");
        assert!(iter.next().is_none(), "line split");

        left_list.push(first.parse().expect("parse int"));
        right_list.push(second.parse().expect("parse int"));
    }

    left_list.sort();
    right_list.sort();

    let mut total_dist = 0;
    let mut right_iter = right_list.iter();
    for left_val in left_list.iter() {
        let right_val = right_iter.next().expect("equal length");
        total_dist += (left_val - right_val).abs();
    }
    assert!(right_iter.next().is_none(), "equal length");

    println!("total distance: {total_dist}");

    let mut right_counts = HashMap::new();
    for val in right_list {
        right_counts.insert(val, right_counts.get(&val).unwrap_or(&0) + 1);
    }

    let similarity_score = left_list.iter().map(|val| {
        val * right_counts.get(val).unwrap_or(&0)
    }).fold(0, |a, b| a+b);

    println!("similarity score: {similarity_score}");
}

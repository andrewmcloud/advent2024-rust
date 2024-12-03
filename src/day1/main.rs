use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

fn parse_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open(file_path)
        .expect("Failed to open file");
    let reader = io::BufReader::new(file);

    reader
    .lines()
    .filter_map(Result::ok)
    .filter_map(|line| {
        let mut parts = line.split_whitespace();
        let l = parts.next()?.parse::<i32>().ok();
        let r = parts.next()?.parse::<i32>().ok();
        Some((l?, r?))
    })
    .unzip()
}

fn part1(left: &mut Vec<i32>, right:  &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left
        .iter()
        .zip(right)
        .map(|(l, r)| (*l - *r).abs())
        .sum()
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for x in right {
        *counter.entry(*x).or_insert(0) += 1
    }

    left
        .iter()
        .map(|&x| (x * counter.get(&x).unwrap_or(&0)))
        .sum()
}

fn main() -> io::Result<()> {
    let (mut left, mut right) = parse_input("../input/day1.txt");
    println!("{}", part1(&mut left, &mut right));
    println!("{}", part2(&left, &right));
    Ok(())
}

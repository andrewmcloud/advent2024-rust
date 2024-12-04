use regex::Regex;
use std::{fs, io};

fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("failed to read file")
}

fn part1(instructions: &String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total: i32 = re.captures_iter(instructions)
        .map(|c| {
            let group1_str = c.get(1).map_or("", |m| m.as_str());
            let group2_str = c.get(2).map_or("", |m| m.as_str());

            let x: i32 = group1_str.parse().unwrap_or(0);
            let y: i32 = group2_str.parse().unwrap_or(0);

            x * y
        })
        .sum();

    total
}

fn part2(instructions: &String) -> i32 {
    let mut enabled = true;
    let mut total: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();
    for c in re.captures_iter(instructions) {
        let m = c.get(0).map_or("", |m| m.as_str());
        match m {
            "don't()" => { enabled = false; }
            "do()" => { enabled = true; }
            _ if enabled => {
                if let (Some(x), Some(y)) = (c.get(1), c.get(2)) {
                    let x: i32 = x.as_str().parse().unwrap();
                    let y: i32 = y.as_str().parse().unwrap();
                    total += x * y;
                }
            }
            _ => {}
        }
    }
    total
}

fn main() -> io::Result<()> {
    let instructions = parse_input("input/day3.txt");
    println!("part1: {:?}", part1(&instructions));
    println!("part2: {:?}", part2(&instructions));
    Ok(())
}
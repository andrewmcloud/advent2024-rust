use std::{fs, io};
fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let input = fs::read_to_string(file_path).expect("failed to read file");
    for line in input.lines() {
        reports.push(line
                         .split_whitespace()
                         .filter_map(|x| x.parse::<i32>().ok())
                         .collect());
    }
    reports
}

fn safe(report: &Vec<i32>) -> bool {
    if report[0] > report[1] {
        report.windows(2)
            .all(|w| (1 <= (w[0] - w[1])) && ((w[0] - w[1]) <= 3))
    } else if report[1] > report[0] {
        report.windows(2)
            .all(|w| (1 <= (w[1] - w[0])) && ((w[1] - w[0]) <= 3))
    } else {
        false
    }
}

fn safe2(report: &Vec<i32>) -> bool {
    (0..report.len())
    .map(|i| {
        let filtered: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &x)| x)
            .collect();
        safe(&filtered)
    }).any(|x| x)
}

fn main() -> io::Result<()> {
    let reports = parse_input("../input/day2.txt");
    let part1 = reports
        .iter()
        .map(|report| { safe(report) })
        .filter(|&x| x)
        .count();

    let part2 = reports
        .iter()
        .map(|report| { safe2(report) })
        .filter(|&x| x)
        .count();

    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
    Ok(())
}
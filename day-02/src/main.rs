use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let day = 02;
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut all_lines: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        all_lines.push(numbers);
    }

    let mut a = 0;

    for line in &all_lines {
        if (is_decreasing(line) || is_increasing(line))
            && differences(line).iter().all(|d| d.abs() <= 3)
        {
            a += 1;
        }
    }

    println!("Advent of Code - Day {day} Answer:{a}",);
}

fn is_increasing(v: &Vec<i32>) -> bool {
    v.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(v: &Vec<i32>) -> bool {
    v.windows(2).all(|w| w[0] > w[1])
}

fn differences(v: &Vec<i32>) -> Vec<i32> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

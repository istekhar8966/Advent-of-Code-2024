use std::fs::read_to_string;

fn main() {
    let day = 02;
    // Read the input

    //let mut result: Vec<String> = Vec::new();
    let location_id: String = read_to_string("input.txt").expect("Failed to read file!");
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    let mut diffrence: Vec<i32> = Vec::new();

    for line in location_id.lines() {
        let lines: Vec<&str> = line.split_whitespace().collect();

        let left = lines[0].parse().unwrap();
        let right = lines[1].parse().unwrap();

        first.push(left);
        second.push(right);
    }
    first.sort();
    second.sort();

    for i in 0..first.len() {
        diffrence.push((first[i] - second[i]).abs());
    }

    let result: i32 = diffrence.iter().sum();

    println!("Advent of Code - Day {day} Answer: {result}");
}

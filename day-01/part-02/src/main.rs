use std::fs::read_to_string;

fn main() {
    let (day, part) = (01, 02);

    let file = read_to_string("input.txt").expect("Failed to read file!");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut score: Vec<i32> = Vec::new();

    for line in file.lines() {
        // Split by whitespaces
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse both number
        if parts.len() == 2 {
            let l = parts[0].parse::<i32>().expect("Failed to parse.");
            let r = parts[1].parse::<i32>().expect("Failed to parse.");

            left.push(l);
            right.push(r);
        }
    }
    for i in 0..left.len() {
        let mut number_of_times: i32 = 0;
        for j in 0..right.len() {
            if left[i] == right[j] {
                number_of_times += 1;
            }
        }
        score.push(left[i] * number_of_times);
    }

    let score_sum: i32 = score.iter().sum();

    println!(
        "Advent of Code - Day {} Part {part} Answer: {}",
        day, score_sum
    );
}

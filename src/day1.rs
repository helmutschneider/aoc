use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day1_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut prev = i64::MAX;
    let mut incr_count: usize = 0;
    for line in input.lines() {
        let parsed = parse_i64(line);
        incr_count += usize::from(parsed > prev);
        prev = parsed;
    }
    println!("Day 1A: {:?}", incr_count);
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut prev = i64::MAX;
    let mut incr_count: usize = 0;
    for i in 0..lines.len() {
        let window = parse_i64(lines[i])
            + parse_i64(lines.get(i + 1).unwrap_or(&"0"))
            + parse_i64(lines.get(i + 2).unwrap_or(&"0"));
        incr_count += usize::from(window > prev);
        prev = window;
    }
    println!("Day 1B: {:?}", incr_count);
}

fn parse_i64(line: &str) -> i64 {
    return line.parse::<i64>().unwrap();
}

use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day6_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn parse_and_calculate_count_for_day(input: &str, day: usize) -> usize {
    let nums: Vec<i8> = input
        .trim()
        .split(',')
        .map(|num| num.parse::<i8>().unwrap())
        .collect();

    let mut fish_by_timer_value = [0usize; 9];

    for t in nums {
        fish_by_timer_value[t as usize] += 1;
    }

    for _ in 0..day {
        let zero_tmp = fish_by_timer_value[0];

        fish_by_timer_value[0] = fish_by_timer_value[1];
        fish_by_timer_value[1] = fish_by_timer_value[2];
        fish_by_timer_value[2] = fish_by_timer_value[3];
        fish_by_timer_value[3] = fish_by_timer_value[4];
        fish_by_timer_value[4] = fish_by_timer_value[5];
        fish_by_timer_value[5] = fish_by_timer_value[6];
        fish_by_timer_value[6] = fish_by_timer_value[7] + zero_tmp;
        fish_by_timer_value[7] = fish_by_timer_value[8];
        fish_by_timer_value[8] = zero_tmp;
    }

    return fish_by_timer_value.iter().sum();
}

fn part1(input: &str) {
    println!("Day 6A: {:?}", parse_and_calculate_count_for_day(input, 80));
}

fn part2(input: &str) {
    println!(
        "Day 6B: {:?}",
        parse_and_calculate_count_for_day(input, 256)
    );
}

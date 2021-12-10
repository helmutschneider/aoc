use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day07_input.txt").unwrap();
    let mut numbers: Vec<i64> = input
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    numbers.sort();

    part1(&numbers);
    part2(&numbers);
}

fn part1(input: &[i64]) {
    assert_eq!(0, input.len() % 2);

    // this is only correct for inputs where len % 2 == 0.
    let median_index = (input.len() / 2) - 1;
    let x = input[median_index];
    let mut fuel: i64 = 0;

    for num in input {
        fuel += (x - num).abs();
    }

    println!("Day 7A: {:?}", fuel);
}

fn determine_fuel_cost(num: i64) -> i64 {
    let mut sum = 0;
    for i in 0..=num {
        sum += i;
    }
    return sum;
}

fn part2(input: &[i64]) {
    let min = input[0];
    let max = input[input.len() - 1];
    let mut min_fuel = i64::MAX;

    for maybe_target_x in min..=max {
        let mut fuel = 0;
        for num in input {
            fuel += determine_fuel_cost((maybe_target_x - num).abs());

            // HIGH PERFORMANCE TWEAK!!!
            if fuel > min_fuel {
                break;
            }
        }
        min_fuel = std::cmp::min(min_fuel, fuel);
    }

    println!("Day 7B: {:?}", min_fuel);
}

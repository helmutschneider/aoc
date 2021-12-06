use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day6_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone, Copy)]
struct Timer {
    fish_count: usize,
    value: i8,
}

fn parse_and_calculate_count_for_day(input: &str, day: usize) -> usize {
    let nums: Vec<i8> = input
        .trim()
        .split(',')
        .map(|num| num.parse::<i8>().unwrap())
        .collect();

    let mut timers: Vec<Timer> = (0..9)
        .map(|t| Timer {
            fish_count: 0,
            value: t,
        })
        .collect();

    for t in nums {
        timers[t as usize].fish_count += 1;
    }

    for _ in 0..day {
        let mut add_timers: usize = 0;

        for timer in &mut timers {
            timer.value -= 1;

            if timer.value == -1 {
                timer.value = 6;
                add_timers += timer.fish_count;
            }
        }

        timers.push(Timer {
            fish_count: add_timers,
            value: 8,
        });
        timers = timers.into_iter().filter(|t| t.value >= 0).collect();
    }

    return timers.iter().fold(0, |carry, t| carry + t.fish_count);
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

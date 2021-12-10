pub fn run() {
    let input = std::fs::read_to_string("./src/day1_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let floor = input.chars().fold(0, |carry, chr| {
        carry
            + match chr {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
    });
    println!("Day 1A: {:?}", floor);
}

fn part2(input: &str) {
    let mut floor: i64 = 0;
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let chr = chars[i];
        floor += match chr {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            println!("Day 1B: {:?}", i + 1);
            break;
        }
    }
}

use std::fs;

pub fn run() {
    let input = fs::read_to_string("./src/day03_input.txt").unwrap();
    part1(&input);
    part2(&input);
}

const NUM_BITS: usize = 12;

fn accumulate_bits(lines: &[&str]) -> [i64; NUM_BITS] {
    let mut accumulator = [0; NUM_BITS];

    for line in lines {
        let chrs = line.chars().collect::<Vec<char>>();
        assert_eq!(NUM_BITS, chrs.len());

        for i in 0..chrs.len() {
            accumulator[i] += match chrs[i] {
                '0' => -1,
                '1' => 1,
                _ => 0,
            };
        }
    }

    return accumulator;
}

fn part1(input: &str) {
    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let accum = accumulate_bits(&lines);

    for i in 0..accum.len() {
        assert_ne!(0, accum[i]);

        let gamma_bit = if accum[i] > 0 { 1 } else { 0 };
        let epsilon_bit = if accum[i] < 0 { 1 } else { 0 };
        let shift_to = NUM_BITS - 1 - i;

        gamma = gamma | (gamma_bit << shift_to);
        epsilon = epsilon | (epsilon_bit << shift_to);
    }

    println!("Day 3A: Gamma = {:?}, Epsilon = {:?}", gamma, epsilon);
}

#[derive(Debug, PartialEq)]
enum Kind {
    Oxygen,
    CO2,
}

fn find_line_of_kind(input: &str, kind: Kind) -> &str {
    let mut lines = input.lines().collect::<Vec<&str>>();

    for i in 0..NUM_BITS {
        let accum = accumulate_bits(&lines);

        loop {
            if lines.len() == 1 {
                break;
            }

            let mut did_remove_line = false;

            // if we are looking for oxygen and there are an equal
            // amount of 0/1's we want to keep the 1s. since we are
            // looking for the most common characters this results
            // in a default of "1".
            //
            // when looking for CO2 the result is surprisingly the
            // same. because we are looking for uncommon characters
            // we have to default to "1" again because it is then
            // assumed to be more common.
            let common_char = if accum[i] >= 0 { '1' } else { '0' };

            for k in 0..lines.len() {
                let line = lines[k];
                let chrs = line.chars().collect::<Vec<char>>();
                let is_common = common_char == chrs[i];

                if (kind == Kind::Oxygen && !is_common) || (kind == Kind::CO2 && is_common) {
                    lines.remove(k);
                    did_remove_line = true;
                    break;
                }
            }

            if !did_remove_line {
                break;
            }
        }
    }

    assert_eq!(1, lines.len());

    return lines[0];
}

fn part2(input: &str) {
    let oxy = find_line_of_kind(input, Kind::Oxygen);
    let co2 = find_line_of_kind(input, Kind::CO2);

    println!(
        "Day 3B: Oxygen = {:?}, CO2 = {:?}",
        i64::from_str_radix(oxy, 2).unwrap(),
        i64::from_str_radix(co2, 2).unwrap()
    );
}

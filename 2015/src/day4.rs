use md5;

const PUZZLE_INPUT: &'static str = "iwrupvqb";

pub fn run() {
    let mut did_find_first = false;

    for i in 1..u64::MAX {
        let str = format!("{}{}", PUZZLE_INPUT, i);
        let result = md5::compute(&str);
        let hex = format!("{:x}", result);

        if !did_find_first && hex.starts_with("00000") {
            println!("Day 4A: {}", i);
            did_find_first = true;
        }

        if hex.starts_with("000000") {
            println!("Day 4B: {}", i);
            break;
        }
    }
}

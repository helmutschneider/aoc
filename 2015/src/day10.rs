pub fn run() {
    let input = "3113322113";

    part1(input);
    part2(input);
}

fn look_and_say(input: &str) -> String {
    let mut prev: Option<char> = None;
    let mut count = 0;
    let mut out = String::new();

    for ch in input.chars() {
        if prev.is_none() || Some(ch) == prev {
            count += 1;
        }
        if prev.is_some() && Some(ch) != prev {
            out.push_str(&count.to_string());
            out.push_str(&prev.unwrap().to_string());
            count = 1;
        }
        prev = Some(ch);
    }

    if count > 0 && prev.is_some() {
        out.push_str(&count.to_string());
        out.push_str(&prev.unwrap().to_string());
    }

    return out;
}

fn part1(input: &str) {
    let mut yee = input.to_string();
    for _ in 0..40 {
        yee = look_and_say(&yee);
    }

    println!("Day 10A: {}", yee.len());
}

fn part2(input: &str) {
    let mut yee = input.to_string();
    for _ in 0..50 {
        yee = look_and_say(&yee);
    }

    println!("Day 10B: {}", yee.len());
}
